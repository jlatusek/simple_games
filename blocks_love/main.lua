local tetrominos = require("shapes")
local piece = require("piece")
local conf = require("config")

local pieceStructures = tetrominos.pieceStructures
local inert = {}
local pieceType = 1
local nextPieceType = 2
local pieceRotation = 1
local pieceX = 3
local pieceY = 0
local timer = 0
local timerLimit = 0.3
local sequence = {}
local offsetX = 2
local offsetY = 5

local function canPieceMove(testX, testY, testRotation)
	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local testBlockX = testX + x
			local testBlockY = testY + y
			if
				pieceStructures[pieceType][testRotation][y][x] ~= " "
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

local function newSequence()
	sequence = {}
	for pieceTypeIndex = 1, #pieceStructures do
		local position = love.math.random(#sequence + 1)
		table.insert(sequence, position, pieceTypeIndex)
	end
end

local function newPiece()
	pieceX = 3
	pieceY = 0
	pieceType = nextPieceType
	nextPieceType = love.math.random(1, #pieceStructures)
	pieceRotation = love.math.random(1, #pieceStructures[pieceType] - 1)
end

function love.keypressed(key)
	if key == "x" then
		local testRotation = pieceRotation + 1
		if testRotation > #pieceStructures[pieceType] then
			testRotation = 1
		end
		if canPieceMove(pieceX, pieceY, testRotation) then
			pieceRotation = testRotation
		end
	elseif key == "z" then
		local testRotation = pieceRotation - 1
		if testRotation < 1 then
			testRotation = #pieceStructures[pieceType]
		end
		if canPieceMove(pieceX, pieceY, testRotation) then
			pieceRotation = testRotation
		end
	elseif key == "c" or key == "down" then
		while canPieceMove(pieceX, pieceY + 1, pieceRotation) do
			pieceY = pieceY + 1
		end
		timer = timerLimit
	elseif key == "left" then
		local testX = pieceX - 1
		if canPieceMove(testX, pieceY, pieceRotation) then
			pieceX = testX
		end
	elseif key == "right" then
		local testX = pieceX + 1
		if canPieceMove(testX, pieceY, pieceRotation) then
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

	newSequence()
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
		if canPieceMove(pieceX, testY, pieceRotation) then
			pieceY = testY
		else
			for y = 1, conf.pieceYCount do
				for x = 1, conf.pieceXCount do
					local block =
						pieceStructures[pieceType][pieceRotation][y][x]
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
			if not canPieceMove(pieceX, pieceY, pieceRotation) then
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
			local block = pieceStructures[pieceType][pieceRotation][y][x]
			if block ~= " " then
				piece.draw(block, x + pieceX + offsetX, y + pieceY + offsetY)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block = pieceStructures[nextPieceType][1][y][x]
			if block ~= " " then
				piece.draw(block, x + 5, y + 1, 0.8)
			end
		end
	end

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local block = pieceStructures[pieceType][pieceRotation][y][x]
			local shadow_piece_y = 0
			while canPieceMove(pieceX, shadow_piece_y, pieceRotation) do
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
