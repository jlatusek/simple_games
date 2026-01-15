local gridXCount = 10
local gridYCount = 18

local tetrominos = require("tetromino")

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

local inert = {}
local pieceType = 1
local pieceRotation = 1

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

function love.draw()
	for y = 1, gridYCount do
		for x = 1, gridXCount do
			local block = inert[y][x]
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
	end

  for y = 1, 4 do
        for x = 1, 4 do
            local pieceStructures = tetrominos.pieceStructures
            local block = pieceStructures[pieceType][pieceRotation][y][x]
            if block ~= ' ' then
                local color = colors[block]
                love.graphics.setColor(color)

                local blockSize = 20
                local blockDrawSize = blockSize - 1
                love.graphics.rectangle(
                    'fill',
                    (x - 1) * blockSize,
                    (y - 1) * blockSize,
                    blockDrawSize,
                    blockDrawSize
                )
            end
        end
    end
end
