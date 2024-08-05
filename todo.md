### Design Decisions
- [ ] Are we going to use `image` crate or not? (Is there a faster/accelerated/better alternative?)
    - [ ] Are we going to use rgba or always convert rgba to rgb? (Whats the best way, is there a performance penalty?)

### (First) Setup Basic TDD
- [ ] For "Drop Dep `rsguiauto`"
- [ ] For "Make Lib Crate "UI Controls" - Screen"
- [ ] For "Make Lib Crate "UI Controls" - Interact"
- [ ] For "Make Lib Crate "UI Controls" - Target"

### Drop Dep `rsguiauto`

- [ ] Rewrite `pub fn screenshot(..)` in my own style/impl. it was borrowed from rsguiauto
- [ ] Write my own impl of `screen::size()` from rsguiauto
- [ ] Write my own impl of `on_screen(..)` and call it is_within_screen()
- [ ] Remove from Cargo.toml

### Make Lib Crate "UI Controls"

#### Screen
- [ ] `size() -> (u32, u32)` - Gets the size of the screen
- [ ] `screenshot() -> DynamicImage` - Takes a screenshot of the whole screen
- [ ] `is_within_screen(x: u32, y: u32) -> bool` - Checks if a point is within the screen
- [ ] `find_target(source: &mut DynamicImage, target: &DynamicImage, looseness: f32) -> Option<(u32, u32)>` - Returns the top left corner of the first match of target in source
- [ ] `find_all_targets(source: &mut DynamicImage, target: &DynamicImage, looseness: f32) -> Vec<(u32, u32)>` - Returns the top left corner of all matches of target in source

#### Interact
- [ ] `mouse_position() -> (u32, u32)` - Gets current mouse position
- [ ] `mouse_move_to(x: u32, y: u32)` - Moves mouse to the given coordinates
- [ ] `mouse_move_by(x: i32, y: i32)` - Moves mouse by the given offsets
- [ ] `click()` - Clicks the mouse where it is currently located
- [ ] `double_click()`  - Double clicks where it is currently located
- [ ] `click_at(x: u32, y: u32, return_mouse: bool)`  - Clicks at the given coordinates and returns mouse position
- [ ] `double_click_at(x: u32, y: u32, return_mouse: bool)` - Double clicks at the given coordinates
- [ ] `click_at_target(target: &DynamicImage, looseness: f32, offset: ClickOffset, return_mouse: bool)` - Clicks at the first match of target in the whole screen. Can use an offset with `ClickOffset::Center`, `ClickOffset::TopLeft`, etc...,  `ClickOffset::Percent(u32, u32)` or `ClickOffset::Pixels(u32, u32)`

#### Target
This will be a struct that will contain the target image and other information (e.g. looseness).
- [ ] `new(target: DynamicImage, TargetOptions) -> Self` - Creates a new target
- [ ] `looseness(value: f32)` - Sets the looseness of the target
- [ ] `offset(value: ClickOffset)` - Sets the offset for clicking on the target
- [ ] `return_mouse(value: bool)` - Returns the mouse to its previous position after clicking on the target
- [ ] impl `Screen` functions
- [ ] impl `Interact` functions

### Extras
- [ ] Multi-screen support (beyond primary screen)
- [ ] Hardware accelerated image search