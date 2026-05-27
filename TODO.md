TODO

1. Change do double buffering / rendering (setup new scene, swap scene, delete old scene with free ,not queue_free)

TIL 
Godot process
1. Input processing
2. Scripts run (_process, _physics_process, signals, callbacks)
3. queue_free nodes actually freed here
4. Rendering