local shapes = require("shapes")
local conf = require("config")
local board = require("board")

local timerLimit = 0.3

function love.keypressed(key)
	if key == "x" then
		local testRotation = board.play_tetromino.rotation + 1
		if testRotation > #shapes.pieceStructures[board.play_tetromino.type] then
			testRotation = 1
		end
		board.checkMoveTetromino(board.play_tetromino, { 0, 0 }, testRotation)
	elseif key == "z" then
		local testRotation = board.play_tetromino.rotation - 1
		if testRotation < 1 then
			testRotation = #shapes.pieceStructures[board.play_tetromino.type]
		end
		board.checkMoveTetromino(board.play_tetromino, { 0, 0 }, testRotation)
	elseif key == "c" or key == "down" then
		while
			board.checkMoveTetromino(board.play_tetromino, { 0, 1 }, board.play_tetromino.rotation)
		do
		end
		board.timer = timerLimit
	elseif key == "left" then
		board.checkMoveTetromino(board.play_tetromino, { -1, 0 }, board.play_tetromino.rotation)
	elseif key == "right" then
		board.checkMoveTetromino(board.play_tetromino, { 1, 0 }, board.play_tetromino.rotation)
	elseif key == "q" or key == "escape" then
		love.event.quit()
	elseif key == "r" then
		love.event.quit("restart")
	end
end

function love.load()
	love.window.setMode(
		(conf.gridXCount + conf.offsetX * 2) * conf.blockSize,
		(conf.gridYCount + conf.offsetY * 2) * conf.blockSize,
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
	board.time_tick(timerLimit, dt)
end

function love.draw()
	board.draw()
end
