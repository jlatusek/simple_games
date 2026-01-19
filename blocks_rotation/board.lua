local conf = require("config")
local shapes = require("shapes")
local piece = require("piece")
local tetromino = require("tetromino")
local coordinates = require("coordinates")

local board = {}
board.__index = board
---@type Tetromino
board.play_tetromino = nil
---@type Tetromino
board.nextTetromino = nil

local function newTetro()
	local tmp = board.play_tetromino
	board.play_tetromino = board.nextTetromino
		or tetromino.new(shapes.random_type())
	board.nextTetromino = tetromino.new(shapes.random_type())
	board.play_tetromino.rotation = tmp and tmp.rotation
		or board.play_tetromino.rotation
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
function board.draw()
	local width = love.graphics.getWidth()
	local height = love.graphics.getHeight()
	local rot = board.play_tetromino.rotation

	love.graphics.push()

	-- 1. Move to Screen Center
	love.graphics.translate(width / 2, height / 2)

	-- 2. Rotate the world opposite to the piece rotation
	-- (Rot 1=0deg, 2=90deg, etc.)
	local angle = (rot - 1) * (math.pi / 2)
	love.graphics.rotate(-angle)

	-- 3. Calculate Center of the Current Piece (in Pixels)
	-- We add pieceXCount/2 * blockSize to find the middle of the 4x4 block
	local piecePixelX = (board.play_tetromino.x + conf.offset - 1)
		* conf.blockSize
	local piecePixelY = (board.play_tetromino.y + conf.offset - 1)
		* conf.blockSize

	local centerOffsetX = (conf.pieceXCount * conf.blockSize) / 2
	local centerOffsetY = (conf.pieceYCount * conf.blockSize) / 2

	-- 4. Shift world back so the piece center matches the screen center
	love.graphics.translate(
		-(piecePixelX + centerOffsetX),
		-(piecePixelY + centerOffsetY)
	)

	-- A. Draw Grid
	for y = 1, conf.gridYCount do
		for x = 1, conf.gridXCount do
			piece.draw(board.inert[y][x], x + conf.offset, y + conf.offset)
		end
	end

	-- B. Draw Shadow
	local shadow_tetro = tetromino.clone(board.play_tetromino)
	while
		board.checkMoveTetromino(shadow_tetro, { 0, 1 }, shadow_tetro.rotation)
	do
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[shadow_tetro.type][shadow_tetro.rotation][y][x]
			if block ~= " " then
				piece.draw(
					block,
					x + shadow_tetro.x + conf.offset,
					y + shadow_tetro.y + conf.offset,
					0.4
				)
			end
		end
	end

	-- C. Draw Active Piece
	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[board.play_tetromino.type][board.play_tetromino.rotation][y][x]
			if block ~= " " then
				piece.draw(
					block,
					x + board.play_tetromino.x + conf.offset,
					y + board.play_tetromino.y + conf.offset
				)
			end
		end
	end

	love.graphics.pop()
	love.graphics.print("NEXT", 20, 10, 0, 3, 3)
	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block =
				shapes.pieceStructures[board.nextTetromino.type][1][y][x]
			if block ~= " " then
				-- Draw at a fixed position on screen (e.g., top left)
				piece.draw(block, x + 1, y + 1, 0.8)
			end
		end
	end
end

---comment
---@param dt number
function board.time_tick(dt)
	board.timer = board.timer + dt
	if board.timer >= conf.timerLimit then
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
			board.timer = conf.timerLimit
		end
	end
end

---comment
---@param key string
function board.keypressed(key)
	if key == conf.keys.rotateRight then
		local testRotation = board.play_tetromino.rotation + 1
		if
			testRotation > #shapes.pieceStructures[board.play_tetromino.type]
		then
			testRotation = 1
		end
		board.checkMoveTetromino(board.play_tetromino, { 0, 0 }, testRotation)
	elseif key == conf.keys.rotateLeft then
		local testRotation = board.play_tetromino.rotation - 1
		if testRotation < 1 then
			testRotation = #shapes.pieceStructures[board.play_tetromino.type]
		end
		board.checkMoveTetromino(board.play_tetromino, { 0, 0 }, testRotation)
	elseif key == conf.keys.hardDrop then
		while
			board.checkMoveTetromino(
				board.play_tetromino,
				{ 0, 1 },
				board.play_tetromino.rotation
			)
		do
		end
		board.timer = conf.timerLimit
	elseif key == conf.keys.left then
		board.checkMoveTetromino(
			board.play_tetromino,
			{ -1, 0 },
			board.play_tetromino.rotation
		)
	elseif key == conf.keys.right then
		board.checkMoveTetromino(
			board.play_tetromino,
			{ 1, 0 },
			board.play_tetromino.rotation
		)
	elseif key == conf.keys.quit then
		love.event.quit()
	elseif key == conf.keys.restart then
		love.event.quit("restart")
	end
end

return board
