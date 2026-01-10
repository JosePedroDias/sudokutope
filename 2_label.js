import { XMLParser, XMLBuilder } from "fast-xml-parser";
import * as fs from "fs";

const FROM = "grid4.svg";
const TO = "grid5.svg";
const TO_DATA = 'data.json';

const DATA = {
    cells: [],
    zones: [],
}

function absPathToVertices(d) {
    const commands = d.split(/(?=[MLHVCSQTAZmlhvcsqtaz])/);
    const points = [];
    for (const command of commands) {
        const point = command.slice(1).split(/[\s,]+/).map(Number);
        points.push(point);
    }
    return points;
}

function randomColor() {
    //const letters = '0123456789ABCDEF';
    const letters = '56789ABCDEF';
    let color = '#';
    for (let i = 0; i < 6; i++) {
        color += letters[Math.floor(Math.random() * letters.length)];
    }
    return color;
}

function getCenter(points) {
    const center = points.reduce((acc, [x, y]) => {
        acc[0] += x;
        acc[1] += y;
        return acc;
    }, [0, 0]);
    center[0] /= points.length;
    center[1] /= points.length;
    return center;
}

const tagsToProcess = ['svg', 'path', undefined];
//const s = new Set();

let zoneIdx = 0;
let cellIdx = 0;

function process(nodes) {
    if (!nodes || !Array.isArray(nodes)) return nodes;

    const toAdd = [];

    //for (const node of nodes) {
    for (let i = nodes.length - 1; i >= 0; --i) {
        const node = nodes[i];
        const tag = Object.keys(node).find(key => key !== ':@' && key !== '#text');

        if (!tagsToProcess.includes(tag)) continue;

        //s.add(tag);

        if (tag === 'path') {
            const attrs = tag && node[':@'];
            const fill = attrs['@@fill'];

            if (fill !== 'none') {
                const points = absPathToVertices(attrs['@@d']);
                points.pop();
                const center = getCenter(points);

                const n = {
                    ':@': {
                        '@@x': center[0],
                        '@@y': center[1],
                        '@@id': `label${cellIdx}`,
                        '@@text-anchor': 'middle',
                        '@@dominant-baseline': 'middle',
                        '@@font-size': '13px',
                        '@@font-family': 'Arial',
                    },
                    text: [
                        {
                            '#text': `${cellIdx}`,
                        }
                    ]
                }
                toAdd.push(n);

                DATA.cells.push({
                    id: cellIdx,
                    center,
                    points,
                });

                attrs['@@fill'] = randomColor();
                delete attrs['@@style'];
                attrs['@@id'] = `cell${cellIdx}`;
                cellIdx += 1;
            } else {
                const points = absPathToVertices(attrs['@@d']);
                points.pop();
                const center = getCenter(points);
                DATA.zones.push({
                    id: zoneIdx,
                    center,
                    points,
                });

                attrs['@@id'] = `zone${zoneIdx}`;
                zoneIdx += 1;
            }
        }

        if (node[tag] && Array.isArray(node[tag])) {
            node[tag] = process(node[tag]);
        }
    }

    if (toAdd.length > 0) nodes = nodes.concat(toAdd);

    return nodes;
}

const options = {
    ignoreAttributes: false,
    preserveOrder: true,
    attributeNamePrefix: "@@", // Prefix to easily identify attributes DEFAULT IS @_
    // :@ key where the attributes get stored
};

const parser = new XMLParser(options);
const builder = new XMLBuilder(options);

const xmlContent = fs.readFileSync(FROM, "utf-8");
let o = parser.parse(xmlContent);

o = process(o);

//console.log(s);

const resultSvg = builder.build(o);
fs.writeFileSync(TO, resultSvg);

fs.writeFileSync(TO_DATA, JSON.stringify(DATA, null, 2));
