local gridXCount = 10
local gridYCount = 18

local tetrominos = require("tetromino")

local pieceStructures = tetrominos.pieceStructures
local inert = {}
local pieceType = 1
local pieceRotation = 1
local pieceX = 3
local pieceY = 0
local timer = 0
local pieceXCount = 4
local pieceYCount = 4

local function drawBlock(block, x, y)
	local colors = {
		[" "] = { 0.87, 0.87, 0.87 },
		i = { 0.47, 0.76, 0.94 },
		j = { 0.93, 0.91, 0.42 },
		l = { 0.49, 0.85, 0.76 },
		o = { 0.92, 0.69, 0.47 },
		s = { 0.83, 0.54, 0.93 },
		t = { 0.97, 0.58, 0.77 },
		z = { 0.66, 0.83, 0.46 },
	}
	local color = colors[block]
	love.graphics.setColor(color)

	local blockSize = 20
	local blockDrawSize = blockSize - 1
	love.graphics.rectangle(
		"fill",
		(x - 1) * blockSize,
		(y - 1) * blockSize,
		blockDrawSize,
		blockDrawSize
	)
end

local function canPieceMove(testX, testY, testRotation)
	for y = 1, pieceYCount do
		for x = 1, pieceXCount do
			local testBlockX = testX + x
			local testBlockY = testY + y
			if
				pieceStructures[pieceType][testRotation][y][x] ~= " "
				and (
					testBlockX < 1
					or testBlockX > gridXCount
					or testBlockY > gridYCount
					or inert[testBlockY][testBlockX] ~= " "
				)
			then
				return false
			end
		end
	end
	return true
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
	elseif key == "down" then
		pieceType = pieceType + 1
		if pieceType > #pieceStructures then
			pieceType = 1
		end
		pieceRotation = 1
	elseif key == "up" then
		pieceType = pieceType - 1
		if pieceType < 1 then
			pieceType = #pieceStructures
		end
		pieceRotation = 1
	elseif key == "q" or key == "escape" then
		love.event.quit()
	elseif key == "r" then
		love.event.quit("restart") -- <--- This reboots the game
	end
end

function love.load()
	love.graphics.setBackgroundColor(255, 255, 255)

	inert = {}
	for y = 1, gridYCount do
		inert[y] = {}
		for x = 1, gridXCount do
			inert[y][x] = " "
		end
	end
end

function love.update(dt)
	timer = timer + dt
	if timer >= 0.5 then
		timer = 0
		local testY = pieceY + 1
		if canPieceMove(pieceX, testY, pieceRotation) then
			pieceY = testY
		end
	end
end

function love.draw()
	for y = 1, gridYCount do
		for x = 1, gridXCount do
			drawBlock(inert[y][x], x, y)
		end
	end

	for y = 1, pieceYCount do
		for x = 1, pieceXCount do
			local block = pieceStructures[pieceType][pieceRotation][y][x]
			if block ~= " " then
				drawBlock(block, x + pieceX, y + pieceY)
			end
		end
	end
end
