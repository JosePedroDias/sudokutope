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
                const cellNumber = target.id.replace('cell', '');

                // Change the fill color to red
                target.setAttribute('fill', 'red');

                // Log the cell number
                console.log(cellNumber);
            }
        });
    })
    .catch(error => {
        console.error('Error loading SVG:', error);
    });
