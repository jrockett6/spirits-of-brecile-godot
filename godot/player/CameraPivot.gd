extends Spatial

onready var player = get_parent()
onready var pivot = get_node(".")

var x_rotation = 0
var y_rotation = 0
var tmp_x_rotation = 0
var tmp_y_rotation = 0
var prev_cursor_x = 0
var prev_cursor_y = 0

func _ready():
	Input.set_use_accumulated_input(false)
	$PlayerCamera.add_exception(player)
	
func _physics_process(delta):
	rotate_camera()

func _unhandled_input(event):
	if (event is InputEventMouseMotion and 
	(Input.is_action_pressed("camera_turn") or 
	Input.is_action_pressed("camera_pan"))):
		tmp_x_rotation += event.position.x - prev_cursor_x
		tmp_y_rotation += event.position.y - prev_cursor_y
	if (event is InputEventMouseMotion):
#		$Label2.text = \
#		String(event.position.x - prev_cursor_x) + "\n" + \
#		String(event.position.y - prev_cursor_y) + "\n" + \
#		"x: " + String(event.relative.x) + \
#		", y: " + String(event.relative.y) + "\n" + \
#		String(event.position)
		
		prev_cursor_x = event.position.x
		prev_cursor_y = event.position.y

func rotate_camera():
	var y_scale = .5
	var x_scale = .5
	
	if Input.is_action_just_pressed("camera_turn"):
		player.rotation_degrees.y += pivot.rotation_degrees.y
		reset_tmp_rotation()
	if Input.is_action_pressed("camera_turn"):
		pivot.rotation_degrees.x += tmp_y_rotation * y_scale
		pivot.rotation_degrees.y = 0
		player.rotation_degrees.y -= tmp_x_rotation * x_scale
		reset_tmp_rotation()
	elif Input.is_action_pressed("camera_pan"):
		pivot.rotation_degrees.x += tmp_y_rotation * y_scale
		pivot.rotation_degrees.y -= tmp_x_rotation * x_scale
		reset_tmp_rotation()

func reset_tmp_rotation():
	tmp_x_rotation = 0
	tmp_y_rotation = 0
