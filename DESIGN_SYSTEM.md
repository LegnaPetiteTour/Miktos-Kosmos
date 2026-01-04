# 🎨 Miktos Kosmos - Design System

## Icon Size Reference

### Current Sizes (Fixed!)
```
Main Logo:        64px (w-16 h-16) ██████████████████
Feature Icons:    40px (w-10 h-10) ████████████
Button Icons:     20px (w-5 h-5)  ██████
Inline Icons:     16px (w-4 h-4)  ████
```

### Icon Containers
```
Large Container:  64px (w-16 h-16) with 40px icon inside
Medium Container: 48px (w-12 h-12) with 32px icon inside
Small Container:  32px (w-8 h-8) with 24px icon inside
```

## Color Palette

### Primary (Blue)
- `primary-50`:  #f0f9ff (Very light)
- `primary-100`: #e0f2fe (Light - for backgrounds)
- `primary-200`: #bae6fd
- `primary-300`: #7dd3fc
- `primary-400`: #38bdf8
- `primary-500`: #0ea5e9
- `primary-600`: #0284c7 (Primary - for icons/text)
- `primary-700`: #0369a1 (Dark)
- `primary-800`: #075985
- `primary-900`: #0c4a6e (Very dark)

### Usage
- Backgrounds: primary-50, primary-100
- Icons/Text: primary-600, primary-700
- Buttons: primary-600 (hover: primary-700)

## Typography Scale

```
text-5xl: 48px - Main page title (Miktos Kosmos)
text-4xl: 36px - Unused currently
text-3xl: 30px - Section headers
text-2xl: 24px - Card headers
text-xl:  20px - Feature titles
text-lg:  18px - Button text
text-base:16px - Body text
text-sm:  14px - Secondary text
text-xs:  12px - Fine print
```

## Spacing Scale

```
space-y-16: 64px - Between major sections
space-y-8:  32px - Between cards
space-y-6:  24px - Inside cards (large)
space-y-4:  16px - Inside cards (medium)
space-y-3:  12px - Inside cards (small)
space-y-2:  8px  - Between related items
```

## Component Patterns

### Feature Card
```svelte
<div class="space-y-4">
  <div class="w-16 h-16 bg-primary-100 rounded-2xl flex items-center justify-center">
    <svg class="w-10 h-10 text-primary-600">...</svg>
  </div>
  <h3 class="text-xl font-semibold">Title</h3>
  <p class="text-gray-600">Description</p>
</div>
```

### Primary Button
```svelte
<button class="btn-primary text-lg px-8 py-3 inline-flex items-center">
  <svg class="w-5 h-5 mr-2">...</svg>
  Button Text
</button>
```

### Secondary Button
```svelte
<button class="btn-secondary text-lg px-8 py-3 inline-flex items-center">
  <svg class="w-5 h-5 mr-2">...</svg>
  Button Text
</button>
```

## Layout Grid

### Home Page
```
max-width: 3xl (768px) - Main content card
padding: 8 (32px) - Page padding
grid-cols-3 - Feature cards (desktop)
grid-cols-1 - Feature cards (mobile)
```

### Scan Page
```
max-width: 6xl (1152px) - Full app layout
padding: 8 (32px) - Page padding
```

## Rounded Corners

```
rounded-lg:   8px  - Cards, buttons
rounded-xl:   12px - Cards (main)
rounded-2xl:  16px - Icon containers ⭐
rounded-full: 999px - Circular elements
```

## Shadow System

```
shadow-sm: Subtle - Cards
shadow:    Default - Hover states
shadow-md: Medium - Active cards
shadow-lg: Large - Modals
```

## Quick Reference

### Most Used Classes
```css
/* Containers */
.card = bg-white rounded-xl shadow-sm border border-gray-200 p-6

/* Buttons */
.btn-primary = bg-primary-600 text-white hover:bg-primary-700
.btn-secondary = bg-gray-200 text-gray-800 hover:bg-gray-300

/* Icons */
Large feature icon: w-10 h-10
Icon container: w-16 h-16 bg-primary-100 rounded-2xl

/* Text */
Page title: text-5xl font-bold
Section title: text-3xl font-semibold
Feature title: text-xl font-semibold
Body: text-gray-600
```

---

**Keep this file open while designing!**
