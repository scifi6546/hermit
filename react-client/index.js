

const { app, BrowserWindow } = require('electron')

app.commandLine.appendSwitch ('ignore-certificate-errors');

function createWindow () {
  // Create the browser window.
  let win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
      webPreferences: { webSecurity: false,allowRunningInsecureContent:true },
    }
  })

  // and load the index.html of the app.
  win.loadFile('build/index.html')
}

app.on('ready', createWindow)