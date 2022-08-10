extends Spatial

var x_rotation = 0
var y_rotation = 0

func _ready():
	$PlayerCamera.add_exception(get_parent())
