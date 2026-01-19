local conf = require("config")
local board = require("board")

function love.keypressed(key)
	board.keypressed(key)
end

function love.load()
	conf.windowSize = conf.gridXCount > conf.gridYCount and conf.gridXCount
		or conf.gridYCount
	conf.windowSize = conf.windowSize + conf.offset * 2
	love.window.setMode(
		conf.windowSize * conf.blockSize,
		conf.windowSize * conf.blockSize,
		{
			resizable = true,
			vsync = true,
			minwidth = 400,
			minheight = 300,
		}
	)
	love.graphics.setBackgroundColor(255, 255, 255)
	love.math.setRandomSeed(os.time())

	board.init()
end

function love.update(dt)
	board.time_tick(dt)
end

function love.draw()
	board.draw()
end
