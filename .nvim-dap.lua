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
      return vim.fn.getcwd() .. "/target/debug/maku"
    end,
    args = {
      "--input",
      "${workspaceFolder}/assets/input.json",
      "--output",
      "Users/yusuke/Desktop/output.png",
    },
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
  },
}
