extends Node2D

@onready var square = preload("res://scene/figure/figure_base_block.tscn")
@onready var cross = preload("res://scene/figure/cross.tscn")

var shape_choose = 0
var odd_even_line = 0

func _on_timer_timeout() -> void:
	for cnt in range(odd_even_line, Config.PLAYAREA_BLOCK_WIDTH, 2):
		var obj = square.instantiate()
		obj.global_position = Vector2(60 + 40 * cnt, 20)
		get_tree().root.add_child.call_deferred(obj)
	odd_even_line = (odd_even_line + 1) % 2
