extends Node

var server = TCP_Server.new()
var clientID = 1
var players = {}
var test_scene = load("res://Test.tscn")

func _ready():
	server.listen(7077, "127.0.0.1")
	
	var inst = test_scene.instance()
	print("here we are")
	add_child(inst)
	
	
	
func _process(delta):
	
	if server.is_connection_available():
		var client = server.take_connection()

#		var player_instance = player.instance()
		
#		player_instance.set_name("Bob")
#		add_child(player_instance)
#		Node.get_node
		
	

	
	

#func _ready():

#func _process(delta):
