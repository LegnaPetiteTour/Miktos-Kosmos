# 4-Week Safety Infrastructure Plan 🛡️

## Philosophy: Build Safety BEFORE File Operations

**Goal:** Create bulletproof safety infrastructure so when you add file organization, you can't accidentally destroy user data.

**Principle:** Every file operation must be:
1. **Logged** - Complete audit trail
2. **Reversible** - One-click undo
3. **Previewed** - User sees before applying
4. **Crash-safe** - Recoverable from any failure

---

## Week 1: Operation Logging System

### Goal: Build the foundation for tracking every operation

### Day 1-2: SQLite Schema Design

**Create:** `src-tauri/migrations/001_operations.sql`

```sql
-- Operations table: tracks each batch of operations
CREATE TABLE operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_type TEXT NOT NULL,  -- 'organize', 'copy', 'move', 'delete'
    status TEXT NOT NULL,           -- 'planned', 'in_progress', 'completed', 'failed', 'rolled_back'
    created_at DATETIME NOT NULL,
    started_at DATETIME,
    completed_at DATETIME,
    total_items INTEGER NOT NULL,
    successful_items INTEGER DEFAULT 0,
    failed_items INTEGER DEFAULT 0,
    description TEXT,               -- Human-readable description
    config JSON                     -- Original configuration (strategy, mode, etc.)
);

-- Operation items: individual file operations
CREATE TABLE operation_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_id INTEGER NOT NULL,
    status TEXT NOT NULL,           -- 'pending', 'completed', 'failed', 'rolled_back'
    source_path TEXT NOT NULL,
    destination_path TEXT,
    file_size INTEGER,
    file_hash TEXT,
    error_message TEXT,
    executed_at DATETIME,
    FOREIGN KEY (operation_id) REFERENCES operations(id) ON DELETE CASCADE
);

-- Operation checkpoints: for crash recovery
CREATE TABLE operation_checkpoints (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_id INTEGER NOT NULL,
    checkpoint_type TEXT NOT NULL,  -- 'start', 'progress', 'complete'
    items_processed INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    metadata JSON,
    FOREIGN KEY (operation_id) REFERENCES operations(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_operations_status ON operations(status);
CREATE INDEX idx_operations_created ON operations(created_at);
CREATE INDEX idx_operation_items_operation_id ON operation_items(operation_id);
CREATE INDEX idx_operation_items_status ON operation_items(status);
```

### Day 3: Database Module

**Create:** `src-tauri/src/database.rs`

```rust
use rusqlite::{Connection, params};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    pub id: Option<i64>,
    pub operation_type: String,
    pub status: OperationStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_items: usize,
    pub successful_items: usize,
    pub failed_items: usize,
    pub description: String,
    pub config: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationStatus {
    Planned,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }
    
    pub fn run_migrations(&self) -> Result<(), rusqlite::Error> {
        // Read and execute migration SQL
        let migration = include_str!("../migrations/001_operations.sql");
        self.conn.execute_batch(migration)?;
        Ok(())
    }
    
    pub fn create_operation(&self, op: &Operation) -> Result<i64, rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO operations (
                operation_type, status, created_at, total_items, 
                description, config
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                op.operation_type,
                op.status.to_string(),
                op.created_at.to_rfc3339(),
                op.total_items,
                op.description,
                op.config.to_string(),
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }
    
    pub fn update_operation_status(
        &self, 
        op_id: i64, 
        status: OperationStatus
    ) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "UPDATE operations SET status = ?1 WHERE id = ?2",
            params![status.to_string(), op_id],
        )?;
        Ok(())
    }
    
    // ... more methods
}
```

### Day 4-5: Testing & Integration

**Create:** `src-tauri/tests/database_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_operation() {
        let db = Database::new(":memory:").unwrap();
        db.run_migrations().unwrap();
        
        let op = Operation {
            id: None,
            operation_type: "organize".to_string(),
            status: OperationStatus::Planned,
            created_at: Utc::now(),
            // ... other fields
        };
        
        let op_id = db.create_operation(&op).unwrap();
        assert!(op_id > 0);
    }
    
    #[test]
    fn test_operation_lifecycle() {
        // Test: Planned -> InProgress -> Completed
    }
    
    #[test]
    fn test_rollback() {
        // Test: Completed -> RolledBack
    }
}
```

**Deliverable:** SQLite database with operation logging + tests

---

## Week 2: Preview Mode System

### Goal: Show users what WILL happen before it happens

### Day 1-2: Plan Generator

**Create:** `src-tauri/src/planner.rs`

```rust
use crate::types::*;
use std::path::{Path, PathBuf};

pub struct OrganizationPlanner {
    strategy: OrganizationStrategy,
    mode: OperationMode,
}

#[derive(Debug, Serialize)]
pub struct OrganizationPlan {
    pub destination_root: String,
    pub strategy: OrganizationStrategy,
    pub mode: OperationMode,
    pub folders: Vec<FolderPreview>,
    pub total_files: usize,
    pub total_size: u64,
    pub files_without_dates: usize,
    pub estimated_duration_seconds: u64,
}

impl OrganizationPlanner {
    pub fn new(strategy: OrganizationStrategy, mode: OperationMode) -> Self {
        Self { strategy, mode }
    }
    
    pub fn create_plan(
        &self,
        files: &[FileMetadata],
        destination_root: &str,
    ) -> OrganizationPlan {
        let mut folders: HashMap<String, FolderPreview> = HashMap::new();
        let mut files_without_dates = 0;
        
        for file in files {
            let folder_path = self.determine_folder(file, destination_root);
            
            if file.date_taken.is_none() && file.modified_at.is_none() {
                files_without_dates += 1;
            }
            
            folders
                .entry(folder_path.clone())
                .or_insert_with(|| FolderPreview {
                    path: folder_path,
                    file_count: 0,
                    total_size: 0,
                    files: Vec::new(),
                })
                .add_file(file);
        }
        
        OrganizationPlan {
            destination_root: destination_root.to_string(),
            strategy: self.strategy.clone(),
            mode: self.mode.clone(),
            folders: folders.into_values().collect(),
            total_files: files.len(),
            total_size: files.iter().map(|f| f.file_size).sum(),
            files_without_dates,
            estimated_duration_seconds: self.estimate_duration(files),
        }
    }
    
    fn determine_folder(&self, file: &FileMetadata, root: &str) -> String {
        match &self.strategy {
            OrganizationStrategy::YearMonth => {
                if let Some(date) = file.date_taken.or(file.modified_at) {
                    format!("{}/{}/{:02}", root, date.year(), date.month())
                } else {
                    format!("{}/Unknown", root)
                }
            }
            // ... other strategies
        }
    }
    
    fn estimate_duration(&self, files: &[FileMetadata]) -> u64 {
        // Rough estimate: 0.1 seconds per file
        (files.len() as f64 * 0.1) as u64
    }
}
```

### Day 3-4: Preview UI Component

**Create:** `src/lib/components/PreviewPanel.svelte`

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  export let plan: OrganizationPlan;
  
  let expandedFolders = new Set<string>();
  
  function toggleFolder(path: string) {
    if (expandedFolders.has(path)) {
      expandedFolders.delete(path);
    } else {
      expandedFolders.add(path);
    }
    expandedFolders = expandedFolders;
  }
  
  function formatSize(bytes: number): string {
    // ... format bytes
  }
</script>

<div class="preview-panel">
  <div class="preview-header">
    <h2>Preview: {plan.strategy} Organization</h2>
    <div class="preview-stats">
      <div class="stat">
        <span class="stat-label">Total Files</span>
        <span class="stat-value">{plan.total_files}</span>
      </div>
      <div class="stat">
        <span class="stat-label">Total Size</span>
        <span class="stat-value">{formatSize(plan.total_size)}</span>
      </div>
      <div class="stat">
        <span class="stat-label">Folders</span>
        <span class="stat-value">{plan.folders.length}</span>
      </div>
      {#if plan.files_without_dates > 0}
        <div class="stat warning">
          <span class="stat-label">Without Dates</span>
          <span class="stat-value">{plan.files_without_dates}</span>
        </div>
      {/if}
    </div>
  </div>
  
  <div class="folder-tree">
    {#each plan.folders as folder}
      <div class="folder-item">
        <div class="folder-header" on:click={() => toggleFolder(folder.path)}>
          <span class="folder-icon">{expandedFolders.has(folder.path) ? '📂' : '📁'}</span>
          <span class="folder-path">{folder.path}</span>
          <span class="folder-count">{folder.file_count} files</span>
          <span class="folder-size">{formatSize(folder.total_size)}</span>
        </div>
        
        {#if expandedFolders.has(folder.path)}
          <div class="folder-files">
            {#each folder.files as file}
              <div class="file-item">
                <span class="file-name">{file}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
  
  <div class="preview-actions">
    <button class="btn-secondary" on:click={() => dispatch('cancel')}>
      Cancel
    </button>
    <button class="btn-primary" on:click={() => dispatch('apply', plan)}>
      Apply Changes
    </button>
  </div>
</div>
```

### Day 5: Tauri Command

**Add to** `src-tauri/src/commands.rs`

```rust
#[tauri::command]
pub async fn create_organization_plan(
    files: Vec<FileMetadata>,
    destination: String,
    strategy: OrganizationStrategy,
    mode: OperationMode,
) -> Result<OrganizationPlan, String> {
    let planner = OrganizationPlanner::new(strategy, mode);
    let plan = planner.create_plan(&files, &destination);
    Ok(plan)
}
```

**Deliverable:** Preview system that shows folder structure before applying

---

## Week 3: Undo/Rollback System

### Goal: Every operation can be undone

### Day 1-2: Reverse Operations

**Create:** `src-tauri/src/undo.rs`

```rust
use crate::database::Database;
use std::fs;
use std::path::Path;

pub struct UndoManager {
    db: Database,
}

impl UndoManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub fn can_undo(&self, operation_id: i64) -> Result<bool, rusqlite::Error> {
        let op = self.db.get_operation(operation_id)?;
        Ok(matches!(op.status, OperationStatus::Completed))
    }
    
    pub fn undo_operation(&self, operation_id: i64) -> Result<UndoResult, String> {
        // Get operation details
        let operation = self.db.get_operation(operation_id)
            .map_err(|e| format!("Failed to get operation: {}", e))?;
            
        if !matches!(operation.status, OperationStatus::Completed) {
            return Err("Can only undo completed operations".to_string());
        }
        
        // Get all operation items
        let items = self.db.get_operation_items(operation_id)
            .map_err(|e| format!("Failed to get items: {}", e))?;
        
        let mut undone = 0;
        let mut failed = 0;
        let mut errors = Vec::new();
        
        // Reverse each operation
        for item in items {
            match operation.operation_type.as_str() {
                "copy" => {
                    // Delete the copied file
                    if let Err(e) = fs::remove_file(&item.destination_path.unwrap()) {
                        failed += 1;
                        errors.push(format!("Failed to remove {}: {}", item.destination_path.unwrap(), e));
                    } else {
                        undone += 1;
                        self.db.update_item_status(item.id.unwrap(), "rolled_back")?;
                    }
                }
                "move" => {
                    // Move file back to original location
                    if let Err(e) = fs::rename(
                        &item.destination_path.unwrap(),
                        &item.source_path
                    ) {
                        failed += 1;
                        errors.push(format!("Failed to restore {}: {}", item.source_path, e));
                    } else {
                        undone += 1;
                        self.db.update_item_status(item.id.unwrap(), "rolled_back")?;
                    }
                }
                _ => {}
            }
        }
        
        // Update operation status
        self.db.update_operation_status(operation_id, OperationStatus::RolledBack)?;
        
        Ok(UndoResult {
            undone_items: undone,
            failed_items: failed,
            errors,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UndoResult {
    pub undone_items: usize,
    pub failed_items: usize,
    pub errors: Vec<String>,
}
```

### Day 3-4: Undo UI

**Create:** `src/lib/components/HistoryPanel.svelte`

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  
  let operations: Operation[] = [];
  let selectedOperation: Operation | null = null;
  
  async function loadHistory() {
    operations = await invoke('get_operation_history');
  }
  
  async function undoOperation(opId: number) {
    if (!confirm('Are you sure you want to undo this operation?')) return;
    
    try {
      const result = await invoke('undo_operation', { operationId: opId });
      alert(`Undone ${result.undone_items} items`);
      await loadHistory();
    } catch (error) {
      alert(`Failed to undo: ${error}`);
    }
  }
  
  onMount(loadHistory);
</script>

<div class="history-panel">
  <h2>Operation History</h2>
  
  <div class="operations-list">
    {#each operations as op}
      <div class="operation-card" class:completed={op.status === 'completed'}>
        <div class="op-header">
          <span class="op-type">{op.operation_type}</span>
          <span class="op-status" class:status-{op.status}>{op.status}</span>
        </div>
        <div class="op-description">{op.description}</div>
        <div class="op-stats">
          <span>{op.successful_items} succeeded</span>
          {#if op.failed_items > 0}
            <span class="text-error">{op.failed_items} failed</span>
          {/if}
        </div>
        <div class="op-date">{formatDate(op.created_at)}</div>
        
        {#if op.status === 'completed'}
          <button class="btn-undo" on:click={() => undoOperation(op.id)}>
            Undo
          </button>
        {/if}
      </div>
    {/each}
  </div>
</div>
```

### Day 5: Crash Recovery

**Add to** `src-tauri/src/main.rs`

```rust
#[tauri::command]
async fn check_incomplete_operations(db: State<'_, Database>) -> Result<Vec<Operation>, String> {
    // Find operations with 'in_progress' status
    db.get_incomplete_operations()
        .map_err(|e| format!("Failed to check operations: {}", e))
}

#[tauri::command]
async fn recover_operation(
    operation_id: i64,
    action: String,  // 'resume', 'rollback', 'mark_failed'
    db: State<'_, Database>
) -> Result<(), String> {
    match action.as_str() {
        "resume" => {
            // Try to resume operation
            // TODO: Implement resume logic
        }
        "rollback" => {
            // Rollback partial operation
            let undo_manager = UndoManager::new(db.inner().clone());
            undo_manager.undo_operation(operation_id)?;
        }
        "mark_failed" => {
            // Mark as failed and move on
            db.update_operation_status(operation_id, OperationStatus::Failed)?;
        }
        _ => return Err("Invalid action".to_string()),
    }
    Ok(())
}

// On app startup
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db = Database::new("operations.db")?;
            db.run_migrations()?;
            
            // Check for incomplete operations
            let incomplete = db.get_incomplete_operations()?;
            if !incomplete.is_empty() {
                // Emit event to frontend to show recovery dialog
                app.emit_all("incomplete-operations", &incomplete).ok();
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Deliverable:** Complete undo system with crash recovery

---

## Week 4: Safe File Operations

### Goal: Implement copy/move with full safety

### Day 1-2: Safe File Operations

**Create:** `src-tauri/src/executor.rs`

```rust
use std::fs;
use std::path::Path;
use crate::database::Database;
use crate::types::*;

pub struct OperationExecutor {
    db: Database,
}

impl OperationExecutor {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub async fn execute_plan(
        &self,
        plan: OrganizationPlan,
        app_handle: &AppHandle,
    ) -> Result<OperationResult, String> {
        // Pre-flight checks
        self.validate_plan(&plan)?;
        
        // Create operation record
        let operation = Operation {
            id: None,
            operation_type: match plan.mode {
                OperationMode::Copy => "copy",
                OperationMode::Move => "move",
            }.to_string(),
            status: OperationStatus::Planned,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            total_items: plan.total_files,
            successful_items: 0,
            failed_items: 0,
            description: format!(
                "{} {} files using {}",
                plan.mode, plan.total_files, plan.strategy
            ),
            config: serde_json::to_value(&plan).unwrap(),
        };
        
        let op_id = self.db.create_operation(&operation)?;
        
        // Update status to in_progress
        self.db.update_operation_status(op_id, OperationStatus::InProgress)?;
        
        let mut successful = 0;
        let mut failed = 0;
        let mut processed = 0;
        
        // Execute each operation
        for folder in &plan.folders {
            // Ensure destination folder exists
            fs::create_dir_all(&folder.path)
                .map_err(|e| format!("Failed to create {}: {}", folder.path, e))?;
                
            for file_name in &folder.files {
                // Find original file
                let source = self.find_source_file(file_name)?;
                let destination = Path::new(&folder.path).join(file_name);
                
                // Execute operation
                let result = match plan.mode {
                    OperationMode::Copy => self.safe_copy(&source, &destination),
                    OperationMode::Move => self.safe_move(&source, &destination),
                };
                
                // Record result
                match result {
                    Ok(_) => {
                        successful += 1;
                        self.db.create_operation_item(op_id, &source, &destination, "completed", None)?;
                    }
                    Err(e) => {
                        failed += 1;
                        self.db.create_operation_item(
                            op_id, &source, &destination, "failed", Some(&e.to_string())
                        )?;
                    }
                }
                
                processed += 1;
                
                // Emit progress
                if processed % 10 == 0 {
                    app_handle.emit("operation-progress", &OperationProgress {
                        current_file: file_name.clone(),
                        processed,
                        total: plan.total_files,
                        percentage: (processed as f32 / plan.total_files as f32) * 100.0,
                    }).ok();
                }
            }
        }
        
        // Finalize
        self.db.update_operation_status(op_id, OperationStatus::Completed)?;
        
        Ok(OperationResult {
            success: failed == 0,
            operations: vec![],  // TODO: populate
            successful_count: successful,
            failed_count: failed,
            skipped_count: 0,
            total_size_processed: plan.total_size,
            duration_ms: 0,  // TODO: calculate
            timestamp: Utc::now(),
        })
    }
    
    fn validate_plan(&self, plan: &OrganizationPlan) -> Result<(), String> {
        // Check destination exists and is writable
        let dest = Path::new(&plan.destination_root);
        if !dest.exists() {
            return Err(format!("Destination does not exist: {}", plan.destination_root));
        }
        
        // Check available disk space
        let required_space = plan.total_size;
        let available_space = self.get_available_space(&dest)?;
        
        if available_space < required_space {
            return Err(format!(
                "Not enough disk space. Required: {}, Available: {}",
                required_space, available_space
            ));
        }
        
        Ok(())
    }
    
    fn safe_copy(&self, source: &Path, destination: &Path) -> Result<(), std::io::Error> {
        // Copy with metadata preservation
        fs::copy(source, destination)?;
        Ok(())
    }
    
    fn safe_move(&self, source: &Path, destination: &Path) -> Result<(), std::io::Error> {
        // First copy, then verify, then delete original
        fs::copy(source, destination)?;
        
        // Verify copy succeeded
        let source_hash = self.calculate_hash(source)?;
        let dest_hash = self.calculate_hash(destination)?;
        
        if source_hash != dest_hash {
            fs::remove_file(destination)?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Hash mismatch after copy"
            ));
        }
        
        // Delete original
        fs::remove_file(source)?;
        Ok(())
    }
}
```

### Day 3-4: Testing

**Create comprehensive test suite:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_safe_copy() {
        let temp_dir = tempdir().unwrap();
        // Create test file
        // Execute copy
        // Verify both exist
        // Verify contents match
    }
    
    #[test]
    fn test_safe_move() {
        // Test that source is deleted after move
        // Test that hash verification works
    }
    
    #[test]
    fn test_disk_space_check() {
        // Test that operation fails if not enough space
    }
    
    #[test]
    fn test_crash_recovery() {
        // Simulate crash mid-operation
        // Verify rollback works
    }
    
    #[test]
    fn test_undo_copy() {
        // Execute copy operation
        // Undo it
        // Verify destination files are deleted
    }
    
    #[test]
    fn test_undo_move() {
        // Execute move operation
        // Undo it
        // Verify files are back in original location
    }
}
```

### Day 5: Integration & Polish

1. **Wire everything together**
   - Connect all UI components
   - Test end-to-end workflows
   - Fix bugs

2. **Documentation**
   - Update README with safety features
   - Document recovery procedures
   - Write user guide for undo

3. **Safety Checklist**
   - [ ] All operations logged in SQLite
   - [ ] Preview mode working
   - [ ] Undo tested with copy/move
   - [ ] Crash recovery tested
   - [ ] Disk space checks working
   - [ ] Hash verification on move
   - [ ] Progress tracking accurate

**Deliverable:** Complete, tested safety infrastructure

---

## Success Criteria

At the end of 4 weeks, you should have:

### ✅ Core Safety Features
- SQLite operation log with full audit trail
- Preview mode that shows folder structure
- One-click undo for all operations
- Crash recovery dialog on startup
- Hash verification for move operations
- Disk space validation

### ✅ User Workflows
1. **Safe Copy Workflow**
   - Scan folder → Review results → Create plan → Preview → Apply → Verify
   
2. **Safe Move Workflow**
   - Scan folder → Create plan → Preview (with warnings) → Confirm → Apply → Auto-verify
   
3. **Undo Workflow**
   - View history → Select operation → Click undo → Files restored

4. **Recovery Workflow**
   - App detects incomplete operation → Shows dialog → User chooses: Resume/Rollback/Ignore

### ✅ Testing
- Unit tests for all safety-critical code
- Integration tests for undo/rollback
- Manual test cases documented
- Test dataset with edge cases

### ✅ Documentation
- Updated README with safety guarantees
- User guide for recovery procedures
- Code comments explaining safety mechanisms
- Architecture diagram showing safety flow

---

## After Week 4: File Operations

Once safety infrastructure is solid, you can safely add:

### v0.5 (Week 5-8)
- Date-based organization
- Copy mode (default, safest)
- Move mode (with extra confirmations)
- Batch operations

### v1.0 (Week 9-12)
- Near-duplicate detection
- Configurable thresholds
- Timeline view
- Public beta release

---

## Critical Success Factors

1. **Don't Skip Tests** - Every safety feature MUST have tests
2. **Don't Rush** - Better to take 5 weeks than ship with bugs
3. **Test on Real Data** - Use your own photo library (on a copy!)
4. **Get Feedback** - Have someone else test the undo system
5. **Document Everything** - Future you will thank present you

---

## Weekly Checkpoints

### Week 1 Checkpoint
- [ ] SQLite schema designed and tested
- [ ] Database module with CRUD operations
- [ ] Migration system working
- [ ] Can create/read/update operations

### Week 2 Checkpoint
- [ ] Planner generates accurate previews
- [ ] Preview UI shows folder structure
- [ ] Can expand/collapse folders
- [ ] Statistics accurate

### Week 3 Checkpoint
- [ ] Undo reverses copy operations
- [ ] Undo reverses move operations
- [ ] History panel shows past operations
- [ ] Crash recovery detects incomplete ops

### Week 4 Checkpoint
- [ ] Copy mode works with progress
- [ ] Move mode works with verification
- [ ] Disk space checked before operations
- [ ] All tests passing

---

## Questions to Ask Each Week

### Week 1
- Is the database schema comprehensive enough?
- Are we logging everything we need for undo?
- What edge cases did we miss?

### Week 2
- Does the preview give users confidence?
- Are we showing all important information?
- Can users understand what will happen?

### Week 3
- Does undo feel instant and reliable?
- Are error messages clear?
- What happens if undo fails?

### Week 4
- Are we fast enough for large libraries?
- Is progress tracking accurate?
- Do users feel safe using this?

---

**Start Week 1 on Monday. Let's build this right.** 🛡️

Want me to generate the actual code files for Week 1 so you can get started immediately?
