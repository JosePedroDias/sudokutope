import { XMLParser, XMLBuilder } from "fast-xml-parser";
import * as fs from "fs";

const FROM = "grid3_simp.svg";
const TO = "grid3b.svg";

function removeInvisibleElements(nodes) {
    if (!nodes || !Array.isArray(nodes)) return;

    for (let i = nodes.length - 1; i >= 0; --i) {
        const node = nodes[i];
        total += 1;
        const tag = Object.keys(node).find(key => key !== ':@' && key !== '#text');
        //console.log('tag', tag);
        const attributes = tag && node[':@'];

        if (attributes) {
            //console.log(attributes);
            const opacity = attributes['@@opacity'];
            const style = attributes['@@style'] || "";

            if (opacity === "0" || style.includes("opacity:0") || style.includes("opacity: 0")) {
                removed += 1;
                nodes.splice(i, 1);
                continue;
            }
        }

        if (node[tag] && Array.isArray(node[tag])) {
            removeInvisibleElements(node[tag]);
        }
    }
}

let simplified = 0;

function simpler(n) {
    simplified += 1;
    return Math.round(Number(n) * 10) / 10;
}

function simplifyPath(d) {
    const commands = d.split(/(?=[MLHVCSQTAZmlhvcsqtaz])/);
    const newCommands = [];
    for (const command of commands) {
        const args = command.slice(1).split(/[\s,]+/).map(simpler);
        newCommands.push(command[0] + args.join(" "));
    }
    return newCommands.join("");
}


const attrsToSimplify = ["x", "y", "width", "height", "cx", "cy", "r", "rx", "ry", "x1", "x2", "y1", "y2"];
function simplifyCoordinates(nodes) {
    if (!nodes || !Array.isArray(nodes)) return;

    for (const node of nodes) {
        const tag = Object.keys(node).find(key => key !== ':@' && key !== '#text');
        const attributes = tag && node[':@'];

        if (attributes) {
            const attrKeys = Object.keys(attributes).map(s => s.replace("@@", ""));
            for (const attr of attrKeys) {
                if (attrsToSimplify.includes(attr)) {
                    const escAttr = "@@" + attr;
                    const v = attributes[escAttr];
                    const v2 = simpler(v);
                    attributes[escAttr] = v2;
                    simplified += 1;
                }
            }

            if (attributes['@@d']) {
                attributes['@@d'] = simplifyPath(attributes['@@d']);
            }

            if (attributes['@@opacity']) {
                if (attributes['@@opacity'] === "1") {
                    delete attributes['@@opacity'];
                }   
            }

            if (attributes['@@stroke-width']) {
                if (attributes['@@stroke-width'] === "1") {
                    delete attributes['@@stroke-width'];
                }   
            }

            if (attributes['@@id']) {
                delete attributes['@@id'];
            }

            if (attributes['@@stroke-linejoin']) {
                delete attributes['@@stroke-linejoin'];
            }

            if (attributes['@@style']) {
                let v = attributes['@@style'];
                v = v.replace(/opacity:\s*1/g, "");
                v = v.replace(/stroke-linejoin:\s*round/g, "");
                v = v.replace(/;\s*;/g, ";");
                if (v[0] === ';') v = v.slice(1);
                if (v.length === 0) delete attributes['@@style'];
                else attributes['@@style'] = v;
            }
        }

        if (node[tag] && Array.isArray(node[tag])) {
            simplifyCoordinates(node[tag]);
        }
    }
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
const o = parser.parse(xmlContent);

let total = 0;
let removed = 0;

removeInvisibleElements(o);
console.log(`Removed ${removed} of ${total} elements`);

simplifyCoordinates(o);
console.log(`Simplified ${simplified}`);

const resultSvg = builder.build(o);
fs.writeFileSync(TO, resultSvg);
