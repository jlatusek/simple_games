extends Node2D

var tets = []
var rng = RandomNumberGenerator.new()

func _init() -> void:
	tets = [
		#preload("res://scene/figure/tet_i.tscn"),
		preload("res://scene/figure/tet_j.tscn"),
		#preload("res://scene/figure/tet_l.tscn"),
		#preload("res://scene/figure/tet_o.tscn"),
		#preload("res://scene/figure/tet_s.tscn"),
		#preload("res://scene/figure/tet_t.tscn"),
		#preload("res://scene/figure/tet_z.tscn"),
	]
	
	

func _on_timer_timeout() -> void:
	var figure = rng.randi_range(0, len(tets)-1)
	var obj = tets[figure].instantiate()
	obj.global_position = Vector2(Config.SCENE_WIDTH/2, 20)
	get_tree().root.add_child.call_deferred(obj)
