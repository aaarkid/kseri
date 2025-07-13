# Bevy 0.16 API Guide - Lessons Learned

This guide documents important Bevy API patterns and solutions discovered while building "In the Shadows", particularly focusing on 2D games, WASM deployment, and common pitfalls.

## Table of Contents
1. [Text Rendering in Bevy 0.16](#text-rendering-in-bevy-016)
2. [2D Shapes and Meshes](#2d-shapes-and-meshes)
3. [Camera Systems](#camera-systems)
4. [Resource Management](#resource-management)
5. [Component Patterns](#component-patterns)
6. [WASM-Specific Considerations](#wasm-specific-considerations)
7. [Performance Optimization](#performance-optimization)
8. [Common Pitfalls and Solutions](#common-pitfalls-and-solutions)

## Text Rendering in Bevy 0.16

### Critical API Changes
Bevy 0.16 introduced breaking changes to the text rendering API. The old pattern no longer works:

```rust
// ❌ OLD (pre-0.16) - This won't compile
commands.spawn((
    Text2d("Hello World".to_string()),
    TextColor(Color::WHITE),
    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
));

// ✅ NEW (0.16+) - Correct pattern
commands.spawn((
    Text2d::new("Hello World"),
    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
));
```

### Text Color in 0.16
The `TextColor` component has been removed. Text color is now handled through the Text2d structure itself or through materials.

### Font Loading
Always enable the `default_font` feature in Cargo.toml for WASM builds:

```toml
[dependencies]
bevy = { version = "0.16", features = ["default_font"] }
```

### Text Positioning
Text2d uses world coordinates. For UI-like positioning, calculate based on camera view:

```rust
// Position text relative to screen edges
let screen_x = -300.0;  // Left side
let screen_y = 250.0;   // Top
Transform::from_translation(Vec3::new(screen_x, screen_y, 10.0))  // High Z for layering
```

## 2D Shapes and Meshes

### Basic Shape Creation
Bevy 0.16 provides convenient 2D primitives:

```rust
// Circles
meshes.add(Circle::new(radius))

// Rectangles
meshes.add(Rectangle::new(width, height))

// Regular Polygons (triangles, hexagons, etc.)
meshes.add(RegularPolygon::new(radius, sides))

// Annulus (ring shape) - great for highlighting
meshes.add(Annulus::new(inner_radius, outer_radius))
```

### Mesh2d and Materials
Always pair Mesh2d with MeshMaterial2d:

```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(10.0))),
    MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
    Transform::from_translation(Vec3::new(x, y, z)),
));
```

### Color Creation
Use the new color constructors:
- `Color::srgb(r, g, b)` for standard RGB
- `Color::srgba(r, g, b, a)` for transparency
- `Color::WHITE`, `Color::BLACK` for constants

## Camera Systems

### 2D Camera Setup
```rust
commands.spawn((
    Camera2d,
    Transform::from_xyz(0.0, 0.0, 1000.0),
    CameraController { /* your fields */ },
));
```

### Camera Queries in 0.16
`OrthographicProjection` is no longer a Component in Bevy 0.16:

```rust
// ❌ OLD - Won't work
Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>

// ✅ NEW - Correct pattern
Query<(&mut Transform, &CameraController), With<Camera2d>>
```

### Zoom Implementation
Handle zoom through transform scale:

```rust
if keyboard_input.pressed(KeyCode::KeyQ) {
    transform.scale *= 1.0 + zoom_speed * time.delta_secs();
}
if keyboard_input.pressed(KeyCode::KeyE) {
    transform.scale *= 1.0 - zoom_speed * time.delta_secs();
}
transform.scale = transform.scale.clamp(Vec3::splat(0.5), Vec3::splat(2.0));
```

## Resource Management

### Resource Registration
Always register resources before systems that use them:

```rust
// ❌ WRONG - System uses resource before it's inserted
app.add_systems(Update, system_using_resource)
   .insert_resource(MyResource::new());

// ✅ CORRECT - Resource inserted during Startup
fn setup(mut commands: Commands) {
    commands.insert_resource(MyResource::new());
}
```

### Resource Access in Systems
```rust
// Immutable access
fn my_system(resource: Res<MyResource>) { }

// Mutable access
fn my_system(mut resource: ResMut<MyResource>) { }

// Optional access (won't panic if missing)
fn my_system(resource: Option<Res<MyResource>>) { }
```

## Component Patterns

### Marker Components
Use empty structs for entity identification:

```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

// Query specific entities
fn update_player(query: Query<&Transform, With<Player>>) { }
```

### Component Bundles
Group related components:

```rust
#[derive(Bundle)]
struct NodeBundle {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    visibility: Visibility,
}
```

## WASM-Specific Considerations

### Build Configuration
Essential Cargo.toml settings:

```toml
[dependencies]
bevy = { version = "0.16", features = ["default_font"] }
wasm-bindgen = "0.2"
getrandom = { version = "0.2", features = ["js"] }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit
```

### Random Number Generation
Always use the `js` feature for getrandom in WASM:

```toml
getrandom = { version = "0.2", features = ["js"] }
```

### Text Rendering Issues
Text2d can have rendering issues in WASM (Bevy issue #17670). Workarounds:
1. Ensure proper z-ordering (higher z = on top)
2. Add explicit Visibility components
3. Use the `default_font` feature
4. Consider using UI Text instead of Text2d for static UI

## Performance Optimization

### Entity Spawning
Batch entity spawning when possible:

```rust
// ❌ Inefficient - Multiple separate spawns
for i in 0..100 {
    commands.spawn(/* ... */);
}

// ✅ Better - Use spawn_batch
commands.spawn_batch((0..100).map(|i| {
    (/* components */)
}));
```

### Query Filtering
Use query filters to reduce iteration:

```rust
// Only iterate entities that have changed
Query<&Transform, Changed<Transform>>

// Combine multiple filters
Query<&Transform, (With<Player>, Without<Enemy>)>
```

### System Ordering
Chain systems efficiently:

```rust
app.add_systems(Update, (
    input_system,
    movement_system.after(input_system),
    collision_system.after(movement_system),
    render_system.after(collision_system),
));
```

## Common Pitfalls and Solutions

### 1. Resource Does Not Exist
**Error**: `Resource requested by system does not exist`
**Solution**: Ensure resource is inserted before any system tries to access it

### 2. Text Not Rendering in WASM
**Problem**: Text2d entities don't show up
**Solutions**:
- Update to new Text2d API
- Enable `default_font` feature
- Check z-ordering
- Verify transform positions

### 3. Component Not Found
**Problem**: Query returns empty when entities exist
**Solution**: Ensure all components in the query are actually added to the entity

### 4. Scale vs Size
**Important**: Transform scale affects children. For individual size changes, modify the mesh or sprite size directly.

### 5. Color Space
**Note**: Use `Color::srgb()` for colors that match what you see in design tools. The old `Color::rgb()` is now linear RGB.

## Build and Deploy Commands

```bash
# Development build
cargo build --target wasm32-unknown-unknown

# Release build (optimized)
cargo build --release --target wasm32-unknown-unknown

# Generate web bindings
wasm-bindgen --target web --out-dir web \
  target/wasm32-unknown-unknown/release/your-game.wasm

# Serve locally (requires Python 3)
python3 -m http.server 8000 --directory web
```

## Example: Complete 2D Entity

Here's a complete example combining multiple concepts:

```rust
fn spawn_game_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    // Parent entity with visual
    let parent = commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.2)))),
        Transform::from_translation(position.extend(1.0)),
        MyComponent { /* fields */ },
    )).id();
    
    // Child label
    commands.spawn((
        Text2d::new("Entity Name"),
        Transform::from_translation(Vec3::new(0.0, -30.0, 1.0)),
    )).set_parent(parent);
    
    // Highlight ring
    commands.spawn((
        Mesh2d(meshes.add(Annulus::new(22.0, 25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(
            Color::srgba(1.0, 1.0, 0.0, 0.5)
        ))),
        Transform::from_translation(Vec3::ZERO),
    )).set_parent(parent);
}
```

## Final Tips

1. **Always check Bevy version**: APIs change between versions
2. **Read migration guides**: Bevy provides detailed migration guides for each version
3. **Use cargo check frequently**: Catch API issues early
4. **Test in WASM early**: Don't wait until the end to test WASM builds
5. **Profile performance**: Use Bevy's built-in diagnostics plugins
6. **Handle errors gracefully**: Use Option<Res<T>> for optional resources

This guide reflects the current state of Bevy 0.16. Always consult the official Bevy documentation for the most up-to-date information.