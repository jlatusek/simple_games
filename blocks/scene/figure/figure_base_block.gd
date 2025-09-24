extends Area2D

@onready var IS_FALLING = true

func _physics_process(delta: float) -> void:
	if not $botom_ray.is_colliding() and IS_FALLING:
		self.position.y += Config.FALL_SPEED * delta
	else:
		IS_FALLING = false
		position = position.snapped(Vector2.ONE * 20)
	

func _on_mouse_entered() -> void:
	self.queue_free()


func _on_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	if event.is_action_pressed("left_click"):
		print("Hejka")
		self.queue_free()
