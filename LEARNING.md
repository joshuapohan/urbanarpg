# Godot 4 + Rust Learning Roadmap

Concepts to learn for building a tile-based game (Stardew-like).
Builds on: physics, signals, callbacks, collision, animation frames, Area2D already covered in this project.

---

## Godot Concepts

### Scene Instancing at Runtime
Spawning scenes dynamically via code — essential for crops, enemies, dropped items.
- Docs: https://docs.godotengine.org/en/stable/tutorials/scripting/nodes_and_scene_instances.html
- PackedScene ref: https://docs.godotengine.org/en/stable/classes/class_packedscene.html

### Groups
Tag nodes globally and call methods or find them without casting everywhere.
- Docs: https://docs.godotengine.org/en/stable/tutorials/scripting/groups.html

### Resources
Data objects defined once and reused (crop definitions, item stats, tool configs). Separates data from logic.
- Docs: https://docs.godotengine.org/en/stable/tutorials/scripting/resources.html

### SceneTree
The node hierarchy — switching scenes, pausing the game, finding nodes globally.
- Docs (tutorial): https://docs.godotengine.org/en/stable/tutorials/scripting/scene_tree.html
- Docs (class ref): https://docs.godotengine.org/en/stable/classes/class_scenetree.html

### Autoloads (Singletons)
Global nodes accessible from anywhere — game state, inventory, save manager.
- Docs: https://docs.godotengine.org/en/stable/tutorials/scripting/singletons_autoload.html
- Best practices: https://docs.godotengine.org/en/stable/tutorials/best_practices/autoloads_versus_internal_nodes.html

### TileMap / TileMapLayer
Godot's built-in tile system. Placing, reading and modifying tiles at runtime. Converting world coords to tile coords.
> Note: `TileMap` is deprecated since 4.3, prefer `TileMapLayer`.
- Docs (using tilemaps): https://docs.godotengine.org/en/stable/tutorials/2d/using_tilemaps.html
- Docs (using tilesets): https://docs.godotengine.org/en/stable/tutorials/2d/using_tilesets.html
- TileMapLayer ref: https://docs.godotengine.org/en/stable/classes/class_tilemaplayer.html

### Camera2D
Following the player, setting world bounds, zoom, smoothing.
- Docs: https://docs.godotengine.org/en/stable/classes/class_camera2d.html

### Raycasting
Detecting what's in a direction without a physics body. Useful for interactions, line-of-sight, tools.
- Docs (tutorial): https://docs.godotengine.org/en/stable/tutorials/physics/ray-casting.html
- RayCast2D ref: https://docs.godotengine.org/en/stable/classes/class_raycast2d.html

### Control Nodes (UI)
Godot's UI system — completely separate from Node2D. Hotbars, menus, dialogue boxes, inventory screens.
- Docs (index): https://docs.godotengine.org/en/stable/tutorials/ui/index.html
- Control node gallery: https://docs.godotengine.org/en/stable/tutorials/ui/control_node_gallery.html
- GDQuest UI tutorials: https://www.gdquest.com/tutorial/godot/ui/user-interface-tutorials/

### NavigationAgent2D (Pathfinding)
Moving NPCs around obstacles automatically.
- Docs (overview): https://docs.godotengine.org/en/stable/tutorials/navigation/navigation_introduction_2d.html
- Docs (agents): https://docs.godotengine.org/en/stable/tutorials/navigation/navigation_using_navigationagents.html
- NavigationAgent2D ref: https://docs.godotengine.org/en/stable/classes/class_navigationagent2d.html

### Saving and Loading
Persisting game state to disk and restoring it.
- Docs: https://docs.godotengine.org/en/stable/tutorials/io/saving_games.html

---

## Game Dev Concepts

### State Machines
Managing complex entity states cleanly (player: idle / walking / attacking / interacting).
You're already doing this informally — formalizing it makes it much easier to maintain.
- GDQuest tutorial: https://www.gdquest.com/tutorial/godot/design-patterns/finite-state-machine/

### Coordinate Spaces
World space vs tile space and converting between them. Core to anything grid-based.
```
tile_pos = floor(world_pos / tile_size)
world_pos = tile_pos * tile_size
```
Covered in the TileMap docs above.

### Entity / Component Thinking
How to structure reusable behavior across different entities without copy-pasting logic.
Usually implemented via composition (nodes as components) or Resources.

### UI / Game Separation
Keeping game logic out of UI nodes and vice versa. Signals are the standard bridge — game emits events, UI reacts.

---

## Rust / gdext Specific

- gdext book: https://godot-rust.github.io/book/
- API reference (master): https://godot-rust.github.io/docs/gdext/master/godot/
- Crate docs (docs.rs): https://docs.rs/godot

---

## General Resources

- GDQuest tutorials: https://www.gdquest.com/tutorial/
