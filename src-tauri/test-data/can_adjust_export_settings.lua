return {
	type = "spritesheet",
	content = {
		texture = "../can_adjust_export_settings.png",
		frames = {
			frame_0 = { x = 96, y = 16, w = 16, h = 16, },
			frame_1 = { x = 80, y = 16, w = 16, h = 16, },
			frame_2 = { x = 64, y = 48, w = 16, h = 16, },
			frame_3 = { x = 64, y = 32, w = 16, h = 16, },
			frame_4 = { x = 64, y = 16, w = 16, h = 16, },
			frame_5 = { x = 112, y = 0, w = 16, h = 16, },
			frame_6 = { x = 96, y = 0, w = 16, h = 16, },
			frame_7 = { x = 80, y = 0, w = 16, h = 16, },
			frame_8 = { x = 64, y = 0, w = 16, h = 16, },
			frame_9 = { x = 48, y = 48, w = 16, h = 16, },
			frame_10 = { x = 48, y = 32, w = 16, h = 16, },
			frame_11 = { x = 48, y = 16, w = 16, h = 16, },
			frame_12 = { x = 48, y = 0, w = 16, h = 16, },
			frame_13 = { x = 32, y = 48, w = 16, h = 16, },
			frame_14 = { x = 32, y = 32, w = 16, h = 16, },
			frame_15 = { x = 32, y = 16, w = 16, h = 16, },
			frame_16 = { x = 32, y = 0, w = 16, h = 16, },
			frame_17 = { x = 16, y = 48, w = 16, h = 16, },
			frame_18 = { x = 16, y = 32, w = 16, h = 16, },
			frame_19 = { x = 16, y = 16, w = 16, h = 16, },
			frame_20 = { x = 16, y = 0, w = 16, h = 16, },
			frame_21 = { x = 0, y = 48, w = 16, h = 16, },
			frame_22 = { x = 0, y = 32, w = 16, h = 16, },
			frame_23 = { x = 0, y = 16, w = 16, h = 16, },
			frame_24 = { x = 0, y = 0, w = 16, h = 16, },
		},
		animations = {
			["attack"] = {
				loop = false,
				sequences = {
					{
						direction = "East",
						frames = {
							{
								id = "frame_0", duration = 0.1, ox = 8.0, oy = 16.0,
							},
						},
					},
					{
						direction = "North",
						frames = {
							{
								id = "frame_1", duration = 0.1, ox = 8.0, oy = 16.0,
							},
						},
					},
					{
						direction = "West",
						frames = {
							{
								id = "frame_3", duration = 0.1, ox = 8.0, oy = 16.0,
							},
						},
					},
					{
						direction = "South",
						frames = {
							{
								id = "frame_2", duration = 0.1, ox = 8.0, oy = 16.0,
							},
						},
					},
				},
			},
			["dead"] = {
				loop = false,
				sequences = {
					{
						direction = "North",
						frames = {
							{
								id = "frame_4", duration = 0.1, ox = 8.0, oy = 16.0,
							},
						},
					},
				},
			},
			["idle"] = {
				loop = false,
				sequences = {
					{
						direction = "East",
						frames = {
							{
								id = "frame_5", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -13, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "North",
						frames = {
							{
								id = "frame_6", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -13, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "West",
						frames = {
							{
								id = "frame_8", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -13, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "South",
						frames = {
							{
								id = "frame_7", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -13, w = 10, h = 10 } },
								},
							},
						},
					},
				},
			},
			["walk"] = {
				loop = true,
				sequences = {
					{
						direction = "East",
						frames = {
							{
								id = "frame_9", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_10", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_11", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_12", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "North",
						frames = {
							{
								id = "frame_13", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_14", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_15", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_16", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "West",
						frames = {
							{
								id = "frame_21", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_22", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_23", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_24", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
						},
					},
					{
						direction = "South",
						frames = {
							{
								id = "frame_17", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_18", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_19", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
							{
								id = "frame_20", duration = 0.1, ox = 8.0, oy = 16.0,
								tags = {
									["weak"] = { rect = { x = -5, y = -12, w = 10, h = 10 } },
								},
							},
						},
					},
				},
			},
		},
	},
};
