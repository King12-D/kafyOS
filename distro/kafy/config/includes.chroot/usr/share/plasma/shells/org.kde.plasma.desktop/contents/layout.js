var plasma = getApiVersion(1);

var layout = {
    desktops: []
};

// Create a desktop containment for each screen
for (var i = 0; i < screenCount; ++i) {
    var desktop = new Activity;
    desktop.screen = i;
    desktop.wallpaperPlugin = "org.kde.image";
    desktop.wallpaperMode = "SingleImage";
    
    desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];
    desktop.writeConfig("Image", "kafy");
    
    layout.desktops.push(desktop);
}

// Clear any existing panels
var panels = panelIds;
for (var i = 0; i < panels.length; ++i) {
    var p = panelById(panels[i]);
    if (p) {
        p.remove();
    }
}

// 1. Create Top Panel (Menu, Global Menu, Spacers, Tray, Clock)
var topPanel = new Panel;
topPanel.location = "top";
topPanel.height = Math.round(gridUnit * 1.5);

var kickoff = topPanel.addWidget("org.kde.plasma.kickoff");
kickoff.currentConfigGroup = ["General"];
kickoff.writeConfig("icon", "kafy-logo");

topPanel.addWidget("org.kde.plasma.appmenu");
topPanel.addWidget("org.kde.plasma.panelspacer");
topPanel.addWidget("org.kde.plasma.systemtray");

var clock = topPanel.addWidget("org.kde.plasma.digitalclock");
clock.currentConfigGroup = ["Appearance"];
clock.writeConfig("showSeconds", "false");

// 2. Create Bottom Dock (Floating centered panel with task manager and trash)
var dock = new Panel;
dock.location = "bottom";
dock.height = Math.round(gridUnit * 2.5);
dock.alignment = "center";
dock.hiding = "dodgewindows";

// Enable floating panel mode
dock.currentConfigGroup = ["General"];
dock.writeConfig("floating", "1");

var taskManager = dock.addWidget("org.kde.plasma.icontasks");
taskManager.currentConfigGroup = ["General"];
taskManager.writeConfig("launchers", [
    "applications:org.kde.dolphin.desktop",
    "applications:org.kde.konsole.desktop",
    "applications:firefox-esr.desktop",
    "applications:systemsettings.desktop",
    "applications:org.kde.discover.desktop"
]);

// Add Trash widget at the end of the dock
dock.addWidget("org.kde.plasma.trash");
