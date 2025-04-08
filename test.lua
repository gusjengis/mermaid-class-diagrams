local image_nvim = require("image")
local file_path = "/home/gusjengis/Documents/Code/mermaid-class-diagrams/test.png"
if vim.fn.filereadable(file_path) == 0 then
	vim.notify("File not found: " .. file_path, vim.log.levels.ERROR)
	return
end
local bufnr = vim.api.nvim_get_current_buf()
local winnr = vim.api.nvim_get_current_win()
local image = image_nvim.from_file(file_path, {
	buffer = bufnr,
	window = winnr,
	with_virtual_padding = true,
	inline = true,
	x = 0,
	y = 0,
})

image:render()
