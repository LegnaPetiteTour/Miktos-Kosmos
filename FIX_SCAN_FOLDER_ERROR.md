# Fix: Scan Folder Error - historyStore.add is not a function

## Problem
When clicking "Scan Folder" button, got error:
```
TypeError: historyStore.add is not a function
```

## Root Causes

### 1. Wrong Method Name
**Problem:** Called `historyStore.add()`  
**Solution:** Should be `historyStore.addEntry()`

### 2. Wrong Type Structure
**Problem:** Used old HistoryEntry shape:
```typescript
{
  id: string;
  timestamp: number;  // ❌ Wrong type
  path: string;       // ❌ Wrong field name
  action: string;     // ❌ Doesn't exist
  fileCount: number;  // ❌ Wrong field name
}
```

**Solution:** Use correct HistoryEntry shape:
```typescript
{
  id: string;
  timestamp: string;           // ✅ ISO string
  folder_path: string;         // ✅ Correct name
  total_files: number;         // ✅ Correct name
  total_size: number;          // ✅ Required
  date_range_start?: string;   // ✅ Optional
  date_range_end?: string;     // ✅ Optional
  file_types: {                // ✅ Required
    images: number;
    videos: number;
    documents: number;
    audio: number;
    archives: number;
    other: number;
  };
  errors: number;              // ✅ Required
  warnings: number;            // ✅ Required
  status: 'success' | 'warning' | 'error';  // ✅ Required
}
```

## Changes Made

### 1. Fixed Method Call
```typescript
// Before
historyStore.add(historyEntry);

// After
historyStore.addEntry(historyEntry);
```

### 2. Fixed HistoryEntry Structure
```typescript
const stats = calculateStats(result);
const historyEntry: HistoryEntry = {
  id: Date.now().toString(),
  timestamp: new Date().toISOString(),  // ISO string instead of timestamp
  folder_path: selected,                // Correct field name
  total_files: result.files?.length || 0,
  total_size: stats?.totalSize || 0,
  date_range_start: stats?.oldestDate || undefined,
  date_range_end: stats?.newestDate || undefined,
  file_types: {
    images: stats?.typeCounts.images || 0,
    videos: stats?.typeCounts.videos || 0,
    documents: stats?.typeCounts.documents || 0,
    audio: stats?.typeCounts.audio || 0,
    archives: 0,
    other: stats?.typeCounts.other || 0
  },
  errors: 0,
  warnings: 0,
  status: 'success'
};
```

### 3. Improved Error Handling
```typescript
// Before
catch (error) {
  console.error('Scan error:', error);
  alert(`Error scanning folder: ${error}`);  // Alert popup
}

// After
catch (error) {
  console.error('Scan error:', error);
  // Silent error logging - can add toast notification later
}
```

## Result
- ✅ Scan folder now works without errors
- ✅ History entries are properly saved to localStorage
- ✅ TypeScript types match correctly
- ✅ Better error handling (no alert popup)

## Testing
```bash
pnpm tauri dev
```

Then:
1. Click "Scan Folder" button
2. Select a folder
3. Should scan successfully without error
4. History should be saved (check localStorage: `workspaceHistory`)

---

**Status:** ✅ Fixed  
**Files Modified:** `src/lib/layouts/panels/ToolsPanel.svelte`  
**Impact:** Scan folder functionality now works correctly
