# Visual Tests

This directory contains Puppeteer-based visual tests for the Kseri card rendering system.

## Test Files

### Core Tests

- `test-card-rendering.js` - Main visual rendering test that verifies:
  - Canvas initialization
  - WebGL context availability
  - Bevy initialization (canvas resize)
  - Asset loading (card PNG files)
  - Takes screenshots for visual verification

- `test-pm2-deployment.js` - PM2 deployment test that verifies:
  - PM2 installation
  - Server process status
  - Server accessibility
  - CORS headers configuration
  - Server logs

### Task #5 - Card Rendering System Tests

- `test-card-rendering-system.js` - Comprehensive card rendering system test:
  - Card texture loading verification
  - Card entity spawning with sprites
  - Performance metrics collection
  - Window resize handling with camera scaling
  - Multiple viewport testing (desktop, tablet, mobile)
  - Detailed asset loading statistics

- `test-layout-managers.js` - Layout manager specific tests:
  - **Hand Layout Manager**: Card spacing, hover effects, different hand sizes
  - **Table Layout Manager**: Pile configurations, card stacking
  - **Score Pile Layout Manager**: Accumulation and positioning
  - Responsive layout testing across different screen sizes
  - Z-ordering within each layout area

- `test-z-ordering.js` - Z-ordering system test:
  - Base layer hierarchy verification (score < table < hand < moving)
  - Dynamic z-order within layers
  - Interactive z-order changes (hover, drag)
  - Edge cases and overlap scenarios
  - Consistency and stability checks

## Running Tests

```bash
# Run all visual tests
npm test

# Run individual tests
node tests/visual/test-card-rendering.js
node tests/visual/test-pm2-deployment.js
node tests/visual/test-card-rendering-system.js
node tests/visual/test-layout-managers.js
node tests/visual/test-z-ordering.js

# Run all Task #5 tests
npm run test:task5
```

## Test Output

- Screenshots are saved to `screenshots/` directory:
  - `screenshots/` - General screenshots
  - `screenshots/layouts/` - Layout manager specific screenshots
  - `screenshots/z-ordering/` - Z-ordering test screenshots
- JSON test reports with detailed metrics
- Exit code 0 indicates success
- Exit code 1 indicates failure

## Screenshot Organization

```
screenshots/
├── card-rendering-test.png          # Basic rendering test
├── card-layout-initial.png          # Initial card layout
├── resize-*.png                     # Responsive tests
├── test-results.json                # Test metrics
├── layouts/
│   ├── hand-*.png                   # Hand layout tests
│   ├── table-*.png                  # Table layout tests
│   ├── score-pile-*.png             # Score pile tests
│   ├── responsive-*.png             # Responsive layouts
│   └── layout-test-report.json      # Layout test report
└── z-ordering/
    ├── base-hierarchy-*.png         # Layer hierarchy
    ├── hover-*.png                  # Hover effects
    ├── moving-card-*.png            # Card movement
    ├── consistency-check-*.png      # Stability tests
    └── z-order-test-report.json     # Z-order test report
```

## Visual Verification Guide

When reviewing screenshots, check for:

1. **Card Rendering**: Cards should display with proper textures and scaling
2. **Layout Spacing**: Consistent gaps between cards in hand and table
3. **Z-Ordering**: Proper layering (hand cards above table, moving cards on top)
4. **Hover Effects**: Cards should lift slightly when hovered
5. **Responsive Scaling**: Layouts should adapt to different screen sizes
6. **Performance**: Load times should be reasonable (check test-results.json)

## Requirements

- Node.js with Puppeteer installed
- PM2 installed globally (for deployment test)
- Development server running on port 8001
- Minimum 2GB RAM for Puppeteer with WebGL