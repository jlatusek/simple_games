extends Area2D

@onready var IS_FALLING = true
var tet_block = []
var rays = [
	$botom_ray,
	$right_ray,
	$left_ray,
	$up_ray,
]

#func _init() -> void:
	#for ray: RayCast2D in rays:
		#if ray.is_colliding():
			#var obj = ray.get_collider()
			
	

func _physics_process(delta: float) -> void:
	if $botom_ray.is_colliding():
		var collider = $botom_ray.get_collider()
		var children = self.get_parent().get_children()
		if collider in children:
			self.position.y += Config.FALL_SPEED * delta
		else:
			position = position.snapped(Vector2.ONE * 20)
	else:
		self.position.y += Config.FALL_SPEED * delta
		#IS_FALLING = false
		
	

func _on_mouse_entered() -> void:
	self.queue_free()


func _on_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	if event.is_action_pressed("left_click"):
		self.queue_free()
