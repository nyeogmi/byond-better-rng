/*
	These are simple defaults for your project.
 */

world
	fps = 25		// 25 frames per second
	icon_size = 32	// 32x32 icon size by default

	view = 6		// show up to 6 tiles outward from center (13x13 view)


// Make objects move 8 pixels per tick when walking

mob
	step_size = 8

obj
	step_size = 8

mob
	Login()
		var/string = call("better_rng", "auxtools_init")()
		if (findtext(string, "SUCCESS")) {
			world << "RNG improved!"
		} else {
			world << "RNG not improved: [string]"
		}

		while (1) {
			world << "RNG returned: [rand(100)]"
			world << "Heads or tails?: [pick(0.75;"heads", 0.25;"tails")]"
			world << "Best animal? [pick(list("bat", "bat 2", "robot"))]"
			sleep(1)
		}