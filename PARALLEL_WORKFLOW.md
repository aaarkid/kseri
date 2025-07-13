# Parallel Development Workflow for Kseri Visual Demo

## Current Parallel Tasks (Phase 2)

### Task #6: Implement Card Animation System
- **Branch**: `feature/card-animations`  
- **Worktree**: `../kseri-animations`
- **Status**: Ready to Start
- **Priority**: HIGH for visual polish
- **Dependencies**: Task #5 (Card Rendering) ✓

**Implementation Plan**:
1. **Core Animation System** (Subtask 6.1)
   - Create flexible animation components for position/rotation/scale
   - Implement easing functions (ease-in-out, cubic, bounce)
   - Location: new `systems/animation.rs`

2. **Card Dealing Animations** (Subtask 6.2)
   - Arc motion paths from deck to hands/table
   - Staggered dealing with delays
   - Different arc paths for visual variety

3. **Play-to-Table Animations** (Subtask 6.3)
   - Smooth transition with scaling
   - Subtle bounce effect on landing
   - Hover state animations

4. **Collection Animations** (Subtask 6.4)
   - Swooping motion to score piles
   - Special Kseri effects with glow/particles
   - Point-scoring card highlights

5. **Shuffle & Queue System** (Subtask 6.5)
   - Circular shuffle visualization
   - Animation priority queue
   - Smooth interruption handling

### Task #17: Upgrade UI to Pixel Art Sprites
- **Branch**: `feature/ui-pixel-art`
- **Worktree**: `../kseri-ui-sprites`
- **Status**: Ready to Start
- **Priority**: HIGH for visual consistency
- **Dependencies**: Tasks #5 & #16 ✓

**Implementation Plan**:
1. **UI Sprite Loading System** (Subtask 17.1)
   - Create UIAssets resource for all UI textures
   - Location: new `ui/sprite_loader.rs`

2. **Score Frame Sprites** (Subtask 17.2)
   - Replace Text2d with sprite frames
   - Pixel font rendering inside frames
   - Dynamic score updates

3. **Turn Indicator Arrow** (Subtask 17.3)
   - Animated arrow sprite
   - Smooth rotation/translation to active player
   - 0.5-1.0 second transitions

4. **Kseri Banner System** (Subtask 17.4)
   - Celebratory banner on Kseri captures
   - Fade in/out animations
   - Scale pulse effect

5. **Deck Counter Sprite** (Subtask 17.5)
   - Replace text with sprite frame
   - Card count with pixel font
   - Low-card warning animation

## Completed Tasks (Integrated to Main)

### Phase 1 Completions:
- ✓ **Task #1**: Initialize Rust/Bevy Project Structure
- ✓ **Task #2**: Implement Card Data Structures
- ✓ **Task #3**: Design Core Game State Management
- ✓ **Task #4**: Implement Kseri Game Rules Engine
- ✓ **Task #5**: Create Card Rendering and Visual Assets
- ✓ **Task #8**: Design Game UI and HUD Elements
- ✓ **Task #10**: Build WebSocket Game Server
- ✓ **Task #16**: Create Pixel Art Assets

## Development Commands

```bash
# Work in animation worktree
cd ../kseri-animations
cargo watch -x "check --target wasm32-unknown-unknown"

# Work in UI sprites worktree  
cd ../kseri-ui-sprites
cargo watch -x "check --target wasm32-unknown-unknown"

# Test visual changes (from any worktree)
wasm-pack build --target web && pm2 restart kseri-dev-server
# View at: http://localhost:8000

# Merge when ready
git checkout main
git merge feature/card-animations
git merge feature/ui-pixel-art
```

## Testing Strategy

### Animation Testing (Task #6):
- Visual smoothness at 30/60 FPS
- Animation interruption handling
- Memory cleanup verification
- Timing and easing feel

### UI Sprite Testing (Task #17):
- Text2d removal verification
- Pixel-perfect rendering at 1x/2x scale
- Responsive layout at different resolutions
- Sprite batching performance

## Integration Guidelines

### Coordinate Systems:
- **Z-Layers**: 
  - Table cards: 0-10
  - Hand cards: 20-30
  - Animations: 40-50
  - UI elements: 100+
  - Kseri banner: 200

### Shared Resources:
- Both tasks should use consistent timing (0.3-0.5s for most animations)
- Coordinate on Transform positions to avoid conflicts
- Share easing functions if needed

## Quick Start for Each Task

### Animation Developer:
```rust
// Start with core animation component
#[derive(Component)]
struct Animation {
    start: Vec3,
    end: Vec3,
    duration: f32,
    elapsed: f32,
    easing: EasingFunction,
}
```

### UI Sprite Developer:
```rust
// Start with sprite loader resource
#[derive(Resource)]
struct UIAssets {
    score_frame: Handle<Image>,
    turn_arrow: Handle<Image>,
    kseri_banner: Handle<Image>,
    // etc...
}
```

## Next Parallel Phase (After 6 & 17)

Once animations and UI sprites are complete:
- **Task #7**: Player Input System (depends on #6)
- **Task #9**: WebSocket Client (independent)
- **Task #13**: Deployment Config (independent)

## Notes

- Focus on visual polish for the demo
- Test frequently with `pm2 logs` for debugging
- Coordinate on animation timings for consistency
- Keep performance in mind - target 60 FPS