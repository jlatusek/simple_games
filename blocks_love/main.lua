local shapes = require("shapes")
local piece = require("piece")
local conf = require("config")

local inert = {}
local pieceType = 1
local nextTetroType = 2
local tetroRotation = 1
local timer = 0
local timerLimit = 0.3
local offsetX = 2
local offsetY = 5

local function canPieceMove(testX, testY, testRotation)
	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local testBlockX = testX + x
			local testBlockY = testY + y
			if
				shapes.pieceStructures[pieceType][testRotation][y][x] ~= " "
				and (
					testBlockX < 1
					or testBlockX > conf.gridXCount
					or testBlockY > conf.gridYCount
					or inert[testBlockY][testBlockX] ~= " "
				)
			then
				return false
			end
		end
	end
	return true
end

local function newPiece()
	pieceX = 3
	pieceY = 0
	pieceType = nextTetroType
	nextTetroType = shapes.random_type()
	tetroRotation = 1
end

function love.keypressed(key)
	if key == "x" then
		local testRotation = tetroRotation + 1
		if testRotation > #shapes.pieceStructures[pieceType] then
			testRotation = 1
		end
		if canPieceMove(pieceX, pieceY, testRotation) then
			tetroRotation = testRotation
		end
	elseif key == "z" then
		local testRotation = tetroRotation - 1
		if testRotation < 1 then
			testRotation = #shapes.pieceStructures[pieceType]
		end
		if canPieceMove(pieceX, pieceY, testRotation) then
			tetroRotation = testRotation
		end
	elseif key == "c" or key == "down" then
		while canPieceMove(pieceX, pieceY + 1, tetroRotation) do
			pieceY = pieceY + 1
		end
		timer = timerLimit
	elseif key == "left" then
		local testX = pieceX - 1
		if canPieceMove(testX, pieceY, tetroRotation) then
			pieceX = testX
		end
	elseif key == "right" then
		local testX = pieceX + 1
		if canPieceMove(testX, pieceY, tetroRotation) then
			pieceX = testX
		end
	elseif key == "q" or key == "escape" then
		love.event.quit()
	elseif key == "r" then
		love.event.quit("restart")
	end
end

function love.load()
	love.window.setMode(
		(conf.gridXCount + offsetX * 2) * conf.blockSize,
		(conf.gridYCount + offsetY * 2) * conf.blockSize,
		{
			resizable = true,
			vsync = true,
			minwidth = 400,
			minheight = 300,
		}
	)
	love.graphics.setBackgroundColor(255, 255, 255)
	love.math.setRandomSeed(os.time())

	newPiece()

	inert = {}
	for y = 1, conf.gridYCount do
		inert[y] = {}
		for x = 1, conf.gridXCount do
			inert[y][x] = " "
		end
	end
end

function love.update(dt)
	timer = timer + dt
	if timer >= timerLimit then
		timer = 0
		local testY = pieceY + 1
		if canPieceMove(pieceX, testY, tetroRotation) then
			pieceY = testY
		else
			for y = 1, conf.pieceYCount do
				for x = 1, conf.pieceXCount do
					local block =
						shapes.pieceStructures[pieceType][tetroRotation][y][x]
					if block ~= " " then
						inert[pieceY + y][pieceX + x] = block
					end
				end
			end
			for y = 1, conf.gridYCount do
				local complete = true
				for x = 1, conf.gridXCount do
					if inert[y][x] == " " then
						complete = false
						break
					end
				end

				if complete then
					print("Complete row: " .. y)
					for removeY = y, 2, -1 do
						for removeX = 1, conf.gridXCount do
							inert[removeY][removeX] =
								inert[removeY - 1][removeX]
						end
					end

					for removeX = 1, conf.gridXCount do
						inert[1][removeX] = " "
					end
				end
			end
			newPiece()
			if not canPieceMove(pieceX, pieceY, tetroRotation) then
				love.load()
			end
			timer = timerLimit
		end
	end
end

function love.draw()
	for y = 1, conf.gridYCount do
		for x = 1, conf.gridXCount do
			piece.draw(inert[y][x], x + offsetX, y + offsetY)
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block = shapes.pieceStructures[pieceType][tetroRotation][y][x]
			if block ~= " " then
				piece.draw(block, x + pieceX + offsetX, y + pieceY + offsetY)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block = shapes.pieceStructures[nextTetroType][1][y][x]
			if block ~= " " then
				piece.draw(block, x + 5, y + 1, 0.8)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block = shapes.pieceStructures[pieceType][tetroRotation][y][x]
			local shadow_piece_y = 0
			while canPieceMove(pieceX, shadow_piece_y, tetroRotation) do
				shadow_piece_y = shadow_piece_y + 1
			end
			shadow_piece_y = shadow_piece_y - 1
			if block ~= " " then
				piece.draw(
					block,
					x + pieceX + offsetX,
					y + shadow_piece_y + offsetY,
					0.4
				)
			end
		end
	end
end
