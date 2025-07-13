# Parallel Development Workflow for Kseri Visual Demo

## Git Worktree Setup

Three parallel development branches have been created for concurrent work:

```bash
# Main repository
/home/arkid/DEV/kseri                 [main]

# Task 4: Game Rules Engine
/home/arkid/DEV/kseri-game-rules      [feature/game-rules]

# Task 5: Card Rendering System  
/home/arkid/DEV/kseri-card-rendering  [feature/card-rendering]

# Task 8: Game UI Elements
/home/arkid/DEV/kseri-game-ui         [feature/game-ui]
```

## Task Details and Implementation Plan

### Task 4: Implement Kseri Game Rules Engine (../kseri-game-rules)
**Status**: ~60% Complete  
**Priority**: HIGH  
**What's Already Done**:
- Move validation logic in `systems/validation.rs`
- Card capture mechanics in `turn_management.rs`
- Basic Kseri detection
- Card value scoring (`Card::kseri_value()`)

**What Needs Implementation**:
1. **Initial Dealing System** (Subtask 4.4)
   - Deal 4 cards to each player and 4 to table
   - Ensure no Jacks in initial table cards (reshuffle if needed)
   - Implementation location: `systems/card_management.rs`

2. **Subsequent Dealing** (Subtask 4.4)
   - Deal 4 cards to each player when hands empty
   - Continue until deck exhausted
   - Integration with `state_transitions.rs`

3. **Complete Scoring System** (Subtask 4.5)
   - Add majority cards bonus (+3 points for >26 cards)
   - Implement end-game: remaining table cards to last capturer
   - Complete score aggregation in `GameManager`

### Task 5: Create Card Rendering System (../kseri-card-rendering)
**Status**: ~20% Complete  
**Priority**: CRITICAL for visual demo  
**What's Already Done**:
- Beautiful Greek-themed pixel art cards (64x96) in `assets/cards.rs`
- All 52 cards + card back with meander borders
- Face cards with Greek deity artwork

**What Needs Implementation**:
1. **Texture Loading** (Subtask 5.1)
   - Load generated card images as Bevy textures
   - Create texture handle management system
   - Implementation: new `systems/texture_loader.rs`

2. **Card Entity Spawning** (Subtask 5.2)
   - Create card entities with Sprite components
   - Map Card struct to correct texture
   - Implementation: update `systems/rendering.rs`

3. **Layout Managers** (Subtask 5.3)
   - HandLayout: bottom/top positioning with overlap
   - TableLayout: center pile with card spread
   - ScorePileLayout: collected cards display
   - Implementation: new `systems/layout.rs`

4. **Z-Ordering System** (Subtask 5.4)
   - Dynamic z-order for table pile stacking
   - Selected card brings to front
   - Implementation: add to `systems/rendering.rs`

5. **Window Resize Handling** (Subtask 5.5)
   - Camera scaling system
   - Layout recalculation on resize
   - Implementation: new `systems/camera.rs`

### Task 8: Design Game UI Elements (../kseri-game-ui)
**Status**: ~10% Complete  
**Priority**: MEDIUM  
**What's Already Done**:
- UI pixel art assets in `assets/ui.rs`
- Score frames, buttons, turn indicator, deck counter sprites
- Kseri banner and table texture

**What Needs Implementation**:
1. **UI Entity Hierarchy** (Subtask 8.1)
   - Set up Bevy UI system structure
   - Implementation: new `systems/ui.rs`

2. **Player Names** (Subtask 8.2)
   - Spawn Text2d entities for "Arkid" and "Sofia"
   - Position near score areas

3. **Dynamic Score Display** (Subtask 8.3)
   - Read from Score components
   - Update Text2d when scores change

4. **Turn Indicator** (Subtask 8.4)
   - Use turn indicator sprite from assets
   - Move based on current_player

5. **Deck Counter** (Subtask 8.5)
   - Show remaining cards from Deck component
   - Use deck counter sprite asset

6. **Game Status Messages** (Subtask 8.6)
   - "Kseri!" banner for special captures
   - Round end/Game over messages
   - Fade animations

## Development Commands

```bash
# Start development in each worktree
cd ../kseri-game-rules && cargo watch -x "check --target wasm32-unknown-unknown"
cd ../kseri-card-rendering && cargo watch -x "check --target wasm32-unknown-unknown"  
cd ../kseri-game-ui && cargo watch -x "check --target wasm32-unknown-unknown"

# Run the development server (from main directory)
cd /home/arkid/DEV/kseri
pm2 start ecosystem.config.js
# Access at: http://localhost:8000

# Build and test
wasm-pack build --target web
```

## Testing Approach

1. **Task 4 (Game Rules)**: Unit tests for dealing logic, scoring calculations
2. **Task 5 (Rendering)**: Visual tests with `cargo run` to see cards on screen
3. **Task 8 (UI)**: Visual tests for UI element positioning and updates

## Integration Points

- All tasks read from shared components (`Card`, `Score`, `Player`, etc.)
- Task 5 depends on Task 4's game state for what to render
- Task 8 depends on Task 5's camera/viewport setup
- Coordinate on Transform positions to avoid UI/game overlap

## Next Steps for Each Developer

### Game Rules Developer (Task 4):
1. Start with dealing system implementation
2. Test with unit tests
3. Complete scoring calculations
4. Integrate with existing turn management

### Card Rendering Developer (Task 5):
1. Create texture loading system first
2. Get basic card sprites showing
3. Implement hand layout for visual testing
4. Add table pile with z-ordering

### UI Developer (Task 8):
1. Set up basic Text2d entities
2. Get player names and scores showing
3. Add turn indicator movement
4. Implement game status messages

## Notes

- Card assets are already beautiful - focus on Bevy integration
- Use existing `Text2d` pattern from `main.rs`
- Coordinate z-values: Cards (0-100), UI (200+)
- Test frequently with visual output