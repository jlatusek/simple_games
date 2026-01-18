local conf = require("config")
local shapes = require("shapes")
local piece = require("piece")
local tetromino = require("tetromino")

local board = {}
board.__index = board
---@type Tetromino
board.play_tetromino = nil
---@type Tetromino
board.nextTetromino = nil

local function newTetro()
	board.play_tetromino = board.nextTetromino
		or tetromino.new(shapes.random_type())
	board.nextTetromino = tetromino.new(shapes.random_type())
end

function board.init()
	board.inert = {}
	board.timer = 0
	for y = 1, conf.gridYCount do
		board.inert[y] = {}
		for x = 1, conf.gridXCount do
			board.inert[y][x] = " "
		end
	end
	newTetro()
end

---comment
---@param tetro Tetromino
---@param direction integer[]
---@param testRotation integer
---@return boolean
function board.canTetroMove(tetro, direction, testRotation)
	local testX = tetro.x + direction[1]
	local testY = tetro.y + direction[2]
	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local testBlockX = testX + x
			local testBlockY = testY + y
			if
				shapes.pieceStructures[tetro.type][testRotation][y][x]
					~= " "
				and (
					testBlockX < 1
					or testBlockX > conf.gridXCount
					or testBlockY > conf.gridYCount
					or board.inert[testBlockY][testBlockX] ~= " "
				)
			then
				return false
			end
		end
	end
	return true
end

---comment
---@param tetro Tetromino
---@param direction number[]
---@param testRotation number
---@return boolean
function board.checkMoveTetromino(tetro, direction, testRotation)
	local canMove = board.canTetroMove(tetro, direction, testRotation)
	if canMove then
		tetro.x = tetro.x + direction[1]
		tetro.y = tetro.y + direction[2]
		tetro.rotation = testRotation
	end
	return canMove
end

---Function to draw all blocks creating tetris game
function board.draw()
	for y = 1, conf.gridYCount do
		for x = 1, conf.gridXCount do
			piece.draw(board.inert[y][x], x + conf.offsetX, y + conf.offsetY)
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[board.play_tetromino.type][board.play_tetromino.rotation][y][x]
			if block ~= " " then
				piece.draw(
					block,
					x + board.play_tetromino.x + conf.offsetX,
					y + board.play_tetromino.y + conf.offsetY
				)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[board.nextTetromino.type][1][y][x]
			if block ~= " " then
				piece.draw(block, x + 5, y + 1, 0.8)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[board.play_tetromino.type][board.play_tetromino.rotation][y][x]
            local shadow_tetro = tetromino.clone(board.play_tetromino)
			while
				board.checkMoveTetromino(
					shadow_tetro,
					{ 0, 1 },
					shadow_tetro.rotation
				)
			do
			end
			if block ~= " " then
				piece.draw(
					block,
					x + shadow_tetro.x + conf.offsetX,
					y + shadow_tetro.y + conf.offsetY,
					0.4
				)
			end
		end
	end
end

---comment
---@param timerLimit any
---@param dt any
function board.time_tick(timerLimit, dt)
	board.timer = board.timer + dt
	if board.timer >= timerLimit then
		board.timer = 0
		if
			not board.checkMoveTetromino(
				board.play_tetromino,
				{ 0, 1 },
				board.play_tetromino.rotation
			)
		then
			for y = 1, conf.pieceYCount do
				for x = 1, conf.pieceXCount do
					local block =
						shapes.pieceStructures[board.play_tetromino.type][board.play_tetromino.rotation][y][x]
					if block ~= " " then
						board.inert[board.play_tetromino.y + y][board.play_tetromino.x + x] =
							block
					end
				end
			end
			for y = 1, conf.gridYCount do
				local complete = true
				for x = 1, conf.gridXCount do
					if board.inert[y][x] == " " then
						complete = false
						break
					end
				end

				if complete then
					print("Complete row: " .. y)
					for removeY = y, 2, -1 do
						for removeX = 1, conf.gridXCount do
							board.inert[removeY][removeX] =
								board.inert[removeY - 1][removeX]
						end
					end

					for removeX = 1, conf.gridXCount do
						board.inert[1][removeX] = " "
					end
				end
			end
			newTetro()
			if
				not board.canTetroMove(
					board.play_tetromino,
					{ 0, 0 },
					board.play_tetromino.rotation
				)
			then
				love.load()
			end
			board.timer = timerLimit
		end
	end
end

return board
