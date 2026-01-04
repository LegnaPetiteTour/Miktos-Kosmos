<div class="p-8">
	<div class="max-w-4xl mx-auto space-y-8">
		<!-- Header -->
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Settings</h1>
			<p class="text-gray-600 mt-2">Power without fear</p>
		</div>
		
		<!-- General -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">General</h2>
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Language</label>
					<select class="w-full md:w-64 px-3 py-2 border border-gray-300 rounded-lg">
						<option>English</option>
					</select>
				</div>
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Theme</label>
					<select class="w-full md:w-64 px-3 py-2 border border-gray-300 rounded-lg">
						<option>Light</option>
						<option>Dark</option>
						<option>Auto</option>
					</select>
				</div>
			</div>
		</div>
		
		<!-- Privacy -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Privacy</h2>
			<div class="space-y-4">
				<label class="flex items-center gap-3">
					<input type="checkbox" class="w-4 h-4" />
					<div>
						<div class="font-medium text-gray-900">Anonymous usage statistics</div>
						<div class="text-sm text-gray-600">Help improve Miktos Kosmos (off by default)</div>
					</div>
				</label>
				<label class="flex items-center gap-3">
					<input type="checkbox" checked class="w-4 h-4" />
					<div>
						<div class="font-medium text-gray-900">Local logs</div>
						<div class="text-sm text-gray-600">Keep operation logs on your device</div>
					</div>
				</label>
			</div>
		</div>
		
		<!-- Safety -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Safety</h2>
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">Default Mode</label>
					<select class="w-full md:w-64 px-3 py-2 border border-gray-300 rounded-lg">
						<option>Copy (Safe)</option>
						<option>Move</option>
					</select>
				</div>
				<label class="flex items-center gap-3">
					<input type="checkbox" checked class="w-4 h-4" />
					<div>
						<div class="font-medium text-gray-900">Require confirmation</div>
						<div class="text-sm text-gray-600">Ask before applying changes</div>
					</div>
				</label>
			</div>
		</div>
		
		<!-- Performance -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Performance</h2>
			<div class="space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-2">CPU Threads</label>
					<select class="w-full md:w-64 px-3 py-2 border border-gray-300 rounded-lg">
						<option>Auto</option>
						<option>1</option>
						<option>2</option>
						<option>4</option>
						<option>8</option>
					</select>
				</div>
				<label class="flex items-center gap-3">
					<input type="checkbox" checked class="w-4 h-4" />
					<div>
						<div class="font-medium text-gray-900">Enable caching</div>
						<div class="text-sm text-gray-600">Speed up repeat operations</div>
					</div>
				</label>
			</div>
		</div>
		
		<!-- Rule -->
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-900">
				<strong>Advanced is hidden until needed. Always.</strong> Control without overwhelm.
			</p>
		</div>
	</div>
</div>
