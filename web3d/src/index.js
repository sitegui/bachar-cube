import * as THREE from 'three';
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls.js';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 5);

const renderer = new THREE.WebGLRenderer({alpha: true});
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const smallPieceAngle = Math.PI / 6;

class Cube {
    constructor() {
        this.cubeSide = 1;
        const halfSmallPieceSide = this.cubeSide * Math.tan(smallPieceAngle / 2) / 2;
        const pieceHorizontalBevel = this.cubeSide / 20;
        const pieceVerticalBevel = this.cubeSide / 15;

        const pieceMaterial = new THREE.MeshStandardMaterial({
            color: '#333333',
            roughness: 0.25,
            metalness: 0,
        });

        const whiteMaterial = new THREE.MeshStandardMaterial({color: 'white'});
        const yellowMaterial = new THREE.MeshStandardMaterial({color: 'yellow'});
        const colorMaterials = {
            R: new THREE.MeshStandardMaterial({color: 'red'}),
            B: new THREE.MeshStandardMaterial({color: 'blue'}),
            O: new THREE.MeshStandardMaterial({color: 'orange'}),
            G: new THREE.MeshStandardMaterial({color: 'green'}),
        };

        const buildSmallPiece = (isTop, sideColor) => {
            const name = (isTop ? 'W' : 'Y') + sideColor;
            const topFaceMaterial = isTop ? whiteMaterial : yellowMaterial;
            const sideFaceMaterial = colorMaterials[sideColor];
            return new SmallPiece(halfSmallPieceSide, this.cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, topFaceMaterial, sideFaceMaterial, name);
        };

        const buildBigPiece = (isTop, sideYColor, sideXColor) => {
            const name = (isTop ? 'W' : 'Y') + sideYColor + sideXColor;
            const topFaceMaterial = isTop ? whiteMaterial : yellowMaterial;
            const sideYFaceMaterial = colorMaterials[sideYColor];
            const sideXFaceMaterial = colorMaterials[sideXColor];
            return new BigPiece(halfSmallPieceSide, this.cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, topFaceMaterial, sideYFaceMaterial, sideXFaceMaterial, name);
        };

        const buildMiddlePiece = (frontFaceMaterial, sideFaceMaterial, backFaceMaterial, rotateZ) => {
            const piece = new MiddlePiece(halfSmallPieceSide, this.cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, frontFaceMaterial, sideFaceMaterial, backFaceMaterial);
            piece.mesh.rotateZ(rotateZ);
            piece.mesh.translateZ(-this.cubeSide / 6);
            return piece;
        };

        this.topPieces = [
            buildBigPiece(true, 'R', 'B'),
            buildSmallPiece(true, 'B'),
            buildBigPiece(true, 'B', 'O'),
            buildSmallPiece(true, 'O'),
            buildBigPiece(true, 'O', 'G'),
            buildSmallPiece(true, 'G'),
            buildBigPiece(true, 'G', 'R'),
            buildSmallPiece(true, 'R'),
        ];

        this.middlePieces = [
            buildMiddlePiece(colorMaterials.R, colorMaterials.B, colorMaterials.O, 0),
            buildMiddlePiece(colorMaterials.O, colorMaterials.G, colorMaterials.R, Math.PI),
        ];

        this.bottomPieces = [
            buildSmallPiece(false, 'O'),
            buildBigPiece(false, 'O', 'B'),
            buildSmallPiece(false, 'B'),
            buildBigPiece(false, 'B', 'R'),
            buildSmallPiece(false, 'R'),
            buildBigPiece(false, 'R', 'G'),
            buildSmallPiece(false, 'G'),
            buildBigPiece(false, 'G', 'O'),
        ];

        this.middleSolved = true;

        this.mesh = new THREE.Group();
        for (const piece of this.topPieces) {
            this.mesh.add(piece.mesh);
        }
        for (const piece of this.middlePieces) {
            this.mesh.add(piece.mesh);
        }
        for (const piece of this.bottomPieces) {
            this.mesh.add(piece.mesh);
        }

        this._mixer = new THREE.AnimationMixer(this.mesh);
        this._isMoving = false;

        this.setFromString('WRB,WB,WBO,WO|WOG,WG,WGR,WR true YO,YOB,YB,YBR|YR,YRG,YG,YGO');
    }

    async flip() {
        if (this._isMoving) {
            throw new Error('Only one movement can be applied at a time');
        }
        this._isMoving = true;

        // Update internal representation
        const [flipTop, stayTop] = this._split(this.topPieces);
        const [flipBottom, stayBottom] = this._split(this.bottomPieces);
        this.topPieces = [...flipBottom, ...stayTop];
        this.bottomPieces = [...flipTop, ...stayBottom];
        this.middleSolved = !this.middleSolved;

        const pieces = [...flipTop, this.middlePieces[0], ...flipBottom];

        const angle = Math.PI / 12;
        const axis = new THREE.Vector3(Math.cos(angle), Math.sin(angle), 0);
        const finalQuaternion = new THREE.Quaternion();
        finalQuaternion.setFromAxisAngle(axis, Math.PI);

        await this._animateAndWait(pieces, finalQuaternion);
        this._isMoving = false;
    }

    async rotateTop(steps) {
        await this._rotateTopOrBottom(this.topPieces, -1, steps)
    }

    async rotateBottom(steps) {
        await this._rotateTopOrBottom(this.bottomPieces, 1, steps)
    }

    async _rotateTopOrBottom(layerPieces, direction, steps) {
        if (this._isMoving) {
            throw new Error('Only one movement can be applied at a time');
        }
        this._isMoving = true;

        if (!Number.isInteger(steps) || steps < 1 || steps > 11) {
            this._isMoving = false;
            throw new Error('Invalid number of steps');
        }

        let cut = 0;
        let found = false;
        for (const [i, piece] of layerPieces.entries()) {
            cut += piece.size;

            if (cut === steps) {
                found = true;
                const prefix = layerPieces.splice(0, i + 1);
                layerPieces.push(...prefix);
                break;
            }
        }

        if (!found) {
            this._isMoving = false;
            throw new Error('Invalid number of steps: cannot find a clear cut');
        }

        const axis = new THREE.Vector3(0, 0, 1);
        const finalQuaternion = new THREE.Quaternion();
        finalQuaternion.setFromAxisAngle(axis, direction * steps * Math.PI / 6);

        await this._animateAndWait(layerPieces, finalQuaternion);
        this._isMoving = false;
    }

    toString() {
        function piecesToString(pieces) {
            let totalSize = 0;
            let str = '';
            for (const piece of pieces) {
                str += piece.name;
                totalSize += piece.size;
                if (totalSize === 6) {
                    str += '|';
                } else {
                    str += ',';
                }
            }
            return str.slice(0, -1);
        }

        return piecesToString(this.topPieces) + ' ' + this.middleSolved + ' ' + piecesToString(this.bottomPieces);
    }

    setFromString(str) {
        const [topStr, middleStr, bottomStr] = str.split(' ', 3);
        const allPieces = new Map();
        for (const piece of [...this.topPieces, ...this.bottomPieces]) {
            allPieces.set(piece.name, piece);
        }

        function parseOuterLayer(str) {
            const newPieces = [];
            for (const name of str.split(/[,|]/)) {
                const piece = allPieces.get(name);
                if (!piece) {
                    throw new Error(`Invalid piece: ${name}`);
                }
                allPieces.delete(name);
                newPieces.push(piece);
            }
            return newPieces;
        }

        const newTopPieces = parseOuterLayer(topStr);
        const newBottomPieces = parseOuterLayer(bottomStr);

        if (allPieces.size) {
            throw new Error('Not all pieces were used');
        }

        this.topPieces = newTopPieces;
        this.middleSolved = middleStr === 'true';
        this.bottomPieces = newBottomPieces;

        let slot = 0;
        for (const topPiece of this.topPieces) {
            this._setPiecePosition(topPiece, true, slot);
            slot += topPiece.size;
        }
        slot = 0;
        for (const bottomPiece of this.bottomPieces) {
            this._setPiecePosition(bottomPiece, false, slot);
            slot += bottomPiece.size;
        }

        const middleMesh = this.middlePieces[0].mesh;
        middleMesh.position.set(0, 0, -this.cubeSide / 6);
        middleMesh.setRotationFromQuaternion(new THREE.Quaternion());
        if (!this.middleSolved) {
            middleMesh.rotateX(Math.PI);
        }
    }

    animate(delta) {
        this._mixer.update(delta);
    }

    _setPiecePosition(piece, isTop, slot) {
        const dz = isTop ? this.cubeSide / 6 : -this.cubeSide / 6;
        piece.mesh.setRotationFromQuaternion(new THREE.Quaternion());
        piece.mesh.position.set(0, 0, dz);
        if (isTop) {
            piece.mesh.rotateOnAxis(new THREE.Vector3(0, 0, 1), slot * Math.PI / 6);
        } else {
            piece.mesh.rotateOnAxis(new THREE.Vector3(1, 0, 0), Math.PI);
            piece.mesh.rotateOnAxis(new THREE.Vector3(0, 0, 1), (slot-1) * Math.PI / 6);
        }
    }

    /**
     * Split the pieces into the first and second half
     * @param pieces
     * @private
     */
    _split(pieces) {
        let totalSize = 0;

        for (const [i, piece] of pieces.entries()) {
            totalSize += piece.size;
            if (totalSize === 6) {
                return [pieces.slice(0, i + 1), pieces.slice(i + 1)];
            }
        }

        throw new Error('There must be a prefix that adds up to 6');
    }

    /**
     * Play a given quaternion animation to the given set of pieces, returning a promise that will resolve when it's
     * finished.
     * @param pieces
     * @param finalQuaternion
     * @returns {Promise<unknown>}
     * @private
     */
    _animateAndWait(pieces, finalQuaternion) {
        const group = new THREE.Group();
        this.mesh.add(group);
        for (const piece of pieces) {
            group.attach(piece.mesh);
        }

        const initialQuaternion = new THREE.Quaternion();
        const track = new THREE.QuaternionKeyframeTrack('.quaternion', [0, 1], [
            initialQuaternion.x,
            initialQuaternion.y,
            initialQuaternion.z,
            initialQuaternion.w,
            finalQuaternion.x,
            finalQuaternion.y,
            finalQuaternion.z,
            finalQuaternion.w,
        ]);

        const clip = new THREE.AnimationClip('movement', 1, [track]);
        const action = this._mixer.clipAction(clip, group);
        action.setLoop(THREE.LoopOnce, 0);

        const promise = new Promise(resolve => {
            const finishMovement = event => {
                if (event.action === action) {
                    this._mixer.removeEventListener('finished', finishMovement);

                    group.quaternion.copy(finalQuaternion);
                    for (const child of group.children.slice()) {
                        this.mesh.attach(child);
                    }
                    group.removeFromParent();

                    resolve();
                }
            };
            this._mixer.addEventListener('finished', finishMovement);
        });

        action.play();

        return promise;
    }
}

class SmallPiece {
    constructor(halfSide, cubeSide, verticalBevel, horizontalBevel, mainMaterial, topFaceMaterial, sideFaceMaterial, name) {
        const halfCubeSide = cubeSide / 2;
        const shape = new THREE.Shape();
        shape.moveTo(0, 0);
        shape.lineTo(-halfSide, -halfCubeSide);
        shape.lineTo(halfSide, -halfCubeSide);
        shape.closePath();

        const group = new THREE.Group();

        const sideHeight = cubeSide / 3 - 2 * verticalBevel;
        const geometry = new THREE.ExtrudeGeometry(shape, {
            depth: sideHeight,
            bevelSize: horizontalBevel,
            bevelOffset: -horizontalBevel,
            bevelThickness: verticalBevel,
        });

        // Make three groups out of the two ordinary ones (faces, side) -> (bottom, top, side)
        const geometryGroups = geometry.groups;
        geometry.clearGroups();
        const halfFirstGroupCount = geometryGroups[0].count / 2;
        geometry.addGroup(0, halfFirstGroupCount, 0);
        geometry.addGroup(halfFirstGroupCount, halfFirstGroupCount, 1);
        geometry.addGroup(2 * halfFirstGroupCount, geometryGroups[1].count, 2);

        const piece = new THREE.Mesh(geometry, [mainMaterial, topFaceMaterial, mainMaterial]);
        piece.translateZ(verticalBevel);
        piece.rotateZ(smallPieceAngle);
        group.add(piece);

        const sideFaceGeometry = new THREE.PlaneGeometry(2 * halfSide - 2 * horizontalBevel, sideHeight);
        const sideFace = new THREE.Mesh(sideFaceGeometry, sideFaceMaterial);
        sideFace.rotateZ(smallPieceAngle);
        sideFace.rotateX(Math.PI / 2);
        sideFace.translateZ(halfCubeSide * 1.001);
        sideFace.translateY(halfCubeSide / 3);
        group.add(sideFace);

        this.mesh = group;

        this.size = 1;
        this.name = name;
    }
}

class BigPiece {
    constructor(halfSmallPieceSide, cubeSide, verticalBevel, horizontalBevel, mainMaterial, topFaceMaterial, sideYFaceMaterial, sideXFaceMaterial, name) {
        const halfCubeSide = cubeSide / 2;
        const shape = new THREE.Shape();
        shape.moveTo(0, 0);
        shape.lineTo(halfSmallPieceSide, -halfCubeSide);
        shape.lineTo(halfCubeSide, -halfCubeSide);
        shape.lineTo(halfCubeSide, -halfSmallPieceSide);
        shape.closePath();

        const group = new THREE.Group();

        const sideHeight = cubeSide / 3 - 2 * verticalBevel;
        const geometry = new THREE.ExtrudeGeometry(shape, {
            depth: sideHeight,
            bevelSize: horizontalBevel,
            bevelOffset: -horizontalBevel,
            bevelThickness: verticalBevel,
        });

        // Make three groups out of the two ordinary ones (faces, side) -> (bottom, top, side)
        const geometryGroups = geometry.groups;
        geometry.clearGroups();
        const halfFirstGroupCount = geometryGroups[0].count / 2;
        geometry.addGroup(0, halfFirstGroupCount, 0);
        geometry.addGroup(halfFirstGroupCount, halfFirstGroupCount, 1);
        geometry.addGroup(2 * halfFirstGroupCount, geometryGroups[1].count, 2);

        const piece = new THREE.Mesh(geometry, [mainMaterial, topFaceMaterial, mainMaterial]);
        piece.translateZ(verticalBevel);
        group.add(piece);

        const sideWidth = halfCubeSide - halfSmallPieceSide - 2 * horizontalBevel;
        const sideFaceGeometry = new THREE.PlaneGeometry(sideWidth, sideHeight);

        const sideYFace = new THREE.Mesh(sideFaceGeometry, sideYFaceMaterial);
        sideYFace.rotateX(Math.PI / 2);
        sideYFace.translateZ(halfCubeSide * 1.001);
        sideYFace.translateY(halfCubeSide / 3);
        sideYFace.translateX(halfSmallPieceSide + sideWidth / 2 + horizontalBevel);
        group.add(sideYFace);

        const sideXFace = new THREE.Mesh(sideFaceGeometry, sideXFaceMaterial);
        sideXFace.rotateZ(Math.PI / 2);
        sideXFace.rotateX(Math.PI / 2);
        sideXFace.translateZ(halfCubeSide * 1.001);
        sideXFace.translateY(halfCubeSide / 3);
        sideXFace.translateX(-halfSmallPieceSide - sideWidth / 2 - horizontalBevel);
        group.add(sideXFace);

        this.mesh = group;

        this.size = 2;
        this.name = name;
    }
}

class MiddlePiece {
    constructor(halfSmallPieceSide, cubeSide, verticalBevel, horizontalBevel, mainMaterial, frontFaceMaterial, sideFaceMaterial, backFaceMaterial) {
        const halfCubeSide = cubeSide / 2;
        const shape = new THREE.Shape();
        shape.moveTo(halfSmallPieceSide, -halfCubeSide);
        shape.lineTo(halfCubeSide, -halfCubeSide);
        shape.lineTo(halfCubeSide, halfCubeSide);
        shape.lineTo(-halfSmallPieceSide, halfCubeSide);
        shape.closePath();

        const group = new THREE.Group();

        const sideHeight = cubeSide / 3 - 2 * verticalBevel;
        const geometry = new THREE.ExtrudeGeometry(shape, {
            depth: sideHeight,
            bevelSize: horizontalBevel,
            bevelOffset: -horizontalBevel,
            bevelThickness: verticalBevel,
        });

        const piece = new THREE.Mesh(geometry, mainMaterial);
        piece.translateZ(verticalBevel);
        group.add(piece);

        const frontWidth = halfCubeSide - halfSmallPieceSide - 2 * horizontalBevel;
        const frontFace = new THREE.Mesh(new THREE.PlaneGeometry(frontWidth, sideHeight), frontFaceMaterial);
        frontFace.rotateX(Math.PI / 2);
        frontFace.translateZ(halfCubeSide * 1.001);
        frontFace.translateY(halfCubeSide / 3);
        frontFace.translateX(halfSmallPieceSide + frontWidth / 2 + horizontalBevel);
        group.add(frontFace);

        const sideWidth = cubeSide - 2 * horizontalBevel;
        const sideFace = new THREE.Mesh(new THREE.PlaneGeometry(sideWidth, sideHeight), sideFaceMaterial);
        sideFace.rotateZ(Math.PI / 2);
        sideFace.rotateX(Math.PI / 2);
        sideFace.translateZ(halfCubeSide * 1.001);
        sideFace.translateY(halfCubeSide / 3);
        group.add(sideFace);

        const backWidth = halfCubeSide + halfSmallPieceSide - 2 * horizontalBevel;
        const backFace = new THREE.Mesh(new THREE.PlaneGeometry(backWidth, sideHeight), backFaceMaterial);
        backFace.rotateX(-Math.PI / 2);
        backFace.translateZ(halfCubeSide * 1.001);
        backFace.translateY(-halfCubeSide / 3);
        backFace.translateX(halfCubeSide - backWidth / 2 - horizontalBevel);
        group.add(backFace);

        this.mesh = group;
    }
}

const cube = new Cube();
scene.add(cube.mesh);
cube.setFromString('YO,WG,WBO,WGR|YBR,WO,YGO,YG true WR,WRB,WB,WOG|YR,YRG,YOB,YB');

camera.position.z = 3;

// controls

const controls = new OrbitControls(camera, renderer.domElement);
controls.minDistance = 0.5;
controls.maxDistance = 3;
controls.maxPolarAngle = 3 * Math.PI / 2;

scene.add(new THREE.AmbientLight('white'));

{
    const color = 0xFFFFFF;
    const intensity = 1;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(-1, 2, 6);
    scene.add(light);
}

// helper

// cube.mesh.add(new THREE.AxesHelper(20));
// scene.add(new THREE.AxesHelper(10));

const clock = new THREE.Clock();

function animate() {
    requestAnimationFrame(animate);
    cube.mesh.rotation.z -= 0.003;
    cube.mesh.rotation.x -= 0.001;
    cube.animate(clock.getDelta());
    renderer.render(scene, camera);
}

animate();

document.getElementById('flip').onclick = () => {
    cube.flip();
};
document.getElementById('rotateTop1').onclick = () => {
    cube.rotateTop(1);
};
document.getElementById('rotateTop2').onclick = () => {
    cube.rotateTop(2);
};
document.getElementById('rotateBottom1').onclick = () => {
    cube.rotateBottom(1);
};
document.getElementById('rotateBottom2').onclick = () => {
    cube.rotateBottom(2);
};
document.getElementById('reset').onclick = () => {
    cube.setFromString('WRB,WB,WBO,WO|WOG,WG,WGR,WR true YO,YOB,YB,YBR|YR,YRG,YG,YGO');
};
