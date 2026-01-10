// Cell group definitions
// Each cell belongs to 1 petal (zone) and 2 arms
const cellGroups = {
    // Petal 0 (zone 0) - bottom right petal
    petal0: [35, 14, 5, 13, 33, 34, 6, 15],
    // Petal 1 (zone 1) - bottom left petal
    petal1: [0, 26, 25, 4, 12, 24, 18, 5],
    // Petal 2 (zone 2) - left petal
    petal2: [28, 20, 9, 29, 2, 30, 8, 19],
    // Petal 3 (zone 3) - top petal
    petal3: [22, 10, 21, 28, 23, 11, 3, 31],
    // Petal 4 (zone 4) - right petal
    petal4: [3, 37, 38, 11, 39, 17, 16, 15],

    // Arms along one direction
    arm0: [31, 22, 21, 10, 3, 36, 37, 16],
    arm1: [30, 29, 28, 20, 27, 0, 26, 19],
    arm2: [2, 9, 4, 25, 24, 12, 13, 32],
    arm3: [23, 11, 14, 35, 15, 6, 1, 33],
    arm4: [39, 38, 34, 6, 7, 17, 8, 18],

    // Arms along other direction
    arm5: [31, 30, 2, 23, 39, 38, 11, 22],
    arm6: [22, 29, 9, 11, 38, 37, 3, 21],
    arm7: [21, 28, 4, 14, 34, 6, 15, 10],
    arm8: [10, 20, 25, 35, 6, 1, 16, 3],
    arm9: [3, 27, 0, 15, 1, 33, 13, 36],
    arm10: [36, 0, 19, 16, 7, 8, 18, 37],
    arm11: [37, 12, 18, 17, 8, 2, 30, 24],
    arm12: [24, 5, 13, 33, 32, 12, 4, 9],
};

// Load grid5.svg and add it to the page
fetch('grid5.svg')
    .then(response => response.text())
    .then(svgText => {
        // Parse the SVG text into a DOM element
        const parser = new DOMParser();
        const svgDoc = parser.parseFromString(svgText, 'image/svg+xml');
        const svgElement = svgDoc.documentElement;

        // Add the SVG to the body
        document.body.appendChild(svgElement);

        // Make text elements not receive pointer events
        const textElements = svgElement.querySelectorAll('text');
        textElements.forEach(text => {
            text.style.pointerEvents = 'none';
        });

        // Add click event listener to the SVG
        svgElement.addEventListener('click', (event) => {
            const target = event.target;

            // Check if the clicked element is a cell path
            if (target.tagName === 'path' && target.id && target.id.startsWith('cell')) {
                // Extract the cell number from the id (e.g., "cell5" -> 5)
                const cellNumber = parseInt(target.id.replace('cell', ''));

                // Change the fill color to red
                target.setAttribute('fill', 'red');

                // Log the cell number and its groups
                console.log(`Cell ${cellNumber}`);

                // Find which groups this cell belongs to
                for (const [groupName, cells] of Object.entries(cellGroups)) {
                    if (cells.includes(cellNumber)) {
                        console.log(`  - ${groupName}`);
                    }
                }
            }
        });
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });
