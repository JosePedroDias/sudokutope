import { XMLParser, XMLBuilder } from "fast-xml-parser";
import * as fs from "fs";

const FROM = "grid4.svg";
const TO = "grid5.svg";

function absPathToVertices(d) {
    const commands = d.split(/(?=[MLHVCSQTAZmlhvcsqtaz])/);
    const points = [];
    for (const command of commands) {
        const point = command.slice(1).split(/[\s,]+/).map(Number);
        points.push(point);
    }
    return points;
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
                //console.log(points);

                const center = points.reduce((acc, [x, y]) => {
                    acc[0] += x;
                    acc[1] += y;
                    return acc;
                });
                center[0] /= points.length;
                center[1] /= points.length;
                //console.log(center);

                const n = {
                    text: {
                        '#text': `#${cellIdx}`,
                        ':@': {
                            '@@x': center[0],
                            '@@y': center[1],
                            '@@id': `label${cellIdx}`,
                        },
                    }
                }
                toAdd.push(n);

                attrs['@@id'] = `cell${cellIdx}`;
                cellIdx += 1;
            } else {
                attrs['@@id'] = `zone${zoneIdx}`;
                zoneIdx += 1;
            }
        }

        if (node[tag] && Array.isArray(node[tag])) {
            //process(node[tag]);
            node[tag] = process(node[tag]);
        }
    }

    if (toAdd.length > 0) {
        //console.log(toAdd);
        //nodes.push(...toAdd);
        return nodes.concat(toAdd);
    }
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
