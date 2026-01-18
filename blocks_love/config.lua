local M = {}
M.blockSize = 40
M.blockDrawSize = M.blockSize - 1
M.pieceXCount = 4
M.pieceYCount = 4
M.gridXCount = 10
M.gridYCount = 18
M.offsetX = 2
M.offsetY = 5

M.keys = {
	rotateLeft = "z",
	rotateRight = "x",
	hardDrop = "down",
	left = "left",
	right = "right",
	quit = "q",
	restart = "r",
}

return M
