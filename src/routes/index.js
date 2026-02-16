const MockDevice = "192.169.0.x";
const column_max = 4;
function searchDevices() {
    console.log("Searching for devices...");
    addDevice(MockDevice);
}

function findRow() {
    const deviceList = document.getElementById("device-list");
    const rows = deviceList.children.length;
    let currentRow;
    if (rows > 0) {
        currentRow = deviceList.children[rows - 1];
    } else {
        currentRow = document.createElement("div");
        currentRow.classList.add("device-row");
        deviceList.appendChild(currentRow);
    }

     if (currentRow.children.length >= column_max) {
        currentRow = document.createElement("div");
        currentRow.classList.add("device-row");
        deviceList.appendChild(currentRow);
    }

    return currentRow;
}

function addDevice(device) {
    const deviceElement = createDeviceElement(device);
    const currentRow = findRow();
    currentRow.appendChild(deviceElement);
}

function createDeviceElement(device) {
    const deviceElement = document.createElement("div");
    deviceElement.classList.add("device");
    deviceElement.textContent = device;
    return deviceElement;
}

async function getSystemInfo() {
    console.log("Polling system info...");
    let request = fetch("/api/systeminfo", {
        method: "GET",
        headers: {
            "Authorization": `Bearer ${localStorage.getItem("token")}`
        }
    });
    let response = await request;
    if (response.ok) {
        let data = await response.json();
        console.log("System Info:", data);
    } else {
        console.error("Failed to fetch system info");
    }
}