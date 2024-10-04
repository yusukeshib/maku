local dap = require('dap')

dap.adapters.lldb = {
  type = "executable",
  command = "/opt/homebrew/opt/llvm/bin/lldb-dap",
  name = "lldb",
}

dap.configurations.rust = {
  {
    name = "demo",
    type = "lldb",
    request = "launch",
    program = function()
      return vim.fn.getcwd() .. "/target/debug/demo"
    end,
    args = {
      "--input",
      "${workspaceFolder}/input.png",
      "--output",
      "${workspaceFolder}/output.png",
      "--width",
      "640",
      "--height",
      "640",
    },
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
  },
}
