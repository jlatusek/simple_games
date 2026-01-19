local M = {}

local conf = require("config")

local colors = {
	[" "] = { 0.87, 0.87, 0.87 },
	i = { 0.47, 0.76, 0.94 },
	j = { 0.93, 0.91, 0.42 },
	l = { 0.49, 0.85, 0.76 },
	o = { 0.92, 0.69, 0.47 },
	s = { 0.83, 0.54, 0.93 },
	t = { 0.97, 0.58, 0.77 },
	z = { 0.66, 0.83, 0.46 },
	preview = { 0.75, 0.75, 0.75 },
	shadow = { 0.35, 0.35, 0.35 },
}

function M.draw(block, x, y, transparency)
	local c = colors[block]

	love.graphics.setColor(c[1], c[2], c[3], transparency or 1)

	love.graphics.rectangle(
		"fill",
		(x - 1) * conf.blockSize,
		(y - 1) * conf.blockSize,
		conf.blockDrawSize,
		conf.blockDrawSize
	)
end

return M
