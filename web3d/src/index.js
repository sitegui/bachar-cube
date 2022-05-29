import * as THREE from 'three';
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls.js';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 5);

const renderer = new THREE.WebGLRenderer({alpha: true});
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

class Cube {
    constructor() {
        const cubeSide = 1;
        const smallPieceAngle = Math.PI / 6;
        const halfSmallPieceSide = cubeSide * Math.tan(smallPieceAngle / 2) / 2;
        const pieceHorizontalBevel = cubeSide / 20;
        const pieceVerticalBevel = cubeSide / 15;

        const pieceMaterial = new THREE.MeshStandardMaterial({color: '#333333', roughness: 0.25, metalness: 0});
        const whiteMaterial = new THREE.MeshStandardMaterial({color: 'white'});
        const redMaterial = new THREE.MeshStandardMaterial({color: 'red'});
        const blueMaterial = new THREE.MeshStandardMaterial({color: 'blue'});
        const orangeMaterial = new THREE.MeshStandardMaterial({color: 'orange'});
        const greenMaterial = new THREE.MeshStandardMaterial({color: 'green'});
        const yellowMaterial = new THREE.MeshStandardMaterial({color: 'yellow'});

        function buildSmallPiece(bottomFaceMaterial, topFaceMaterial, sideFaceMaterial, rotateZ, translateZ) {
            const piece = new SmallPiece(halfSmallPieceSide, cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, bottomFaceMaterial, topFaceMaterial, sideFaceMaterial);
            piece.mesh.rotateZ(rotateZ);
            piece.mesh.translateZ(translateZ);
            return piece;
        }

        function buildBigPiece(bottomFaceMaterial, topFaceMaterial, sideYFaceMaterial, sideXFaceMaterial, rotateZ, translateZ) {
            const piece = new BigPiece(halfSmallPieceSide, cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, bottomFaceMaterial, topFaceMaterial, sideYFaceMaterial, sideXFaceMaterial);
            piece.mesh.rotateZ(rotateZ);
            piece.mesh.translateZ(translateZ);
            return piece;
        }

        function buildMiddlePiece(frontFaceMaterial, sideFaceMaterial, backFaceMaterial, rotateZ) {
            const piece = new MiddlePiece(halfSmallPieceSide, cubeSide, pieceVerticalBevel, pieceHorizontalBevel, pieceMaterial, frontFaceMaterial, sideFaceMaterial, backFaceMaterial);
            piece.mesh.rotateZ(rotateZ);
            piece.mesh.translateZ(-cubeSide / 6);
            return piece;
        }

        this.topPieces = [
            buildBigPiece(pieceMaterial, whiteMaterial, redMaterial, blueMaterial, 0, cubeSide / 6),
            buildSmallPiece(pieceMaterial, whiteMaterial, blueMaterial, 3 * smallPieceAngle, cubeSide / 6),
            buildBigPiece(pieceMaterial, whiteMaterial, blueMaterial, orangeMaterial, 3 * smallPieceAngle, cubeSide / 6),
            buildSmallPiece(pieceMaterial, whiteMaterial, orangeMaterial, 6 * smallPieceAngle, cubeSide / 6),
            buildBigPiece(pieceMaterial, whiteMaterial, orangeMaterial, greenMaterial, 6 * smallPieceAngle, cubeSide / 6),
            buildSmallPiece(pieceMaterial, whiteMaterial, greenMaterial, 9 * smallPieceAngle, cubeSide / 6),
            buildBigPiece(pieceMaterial, whiteMaterial, greenMaterial, redMaterial, 9 * smallPieceAngle, cubeSide / 6),
            buildSmallPiece(pieceMaterial, whiteMaterial, redMaterial, 12 * smallPieceAngle, cubeSide / 6)
        ];

        this.middlePieces = [
            buildMiddlePiece(redMaterial, blueMaterial, orangeMaterial, 0),
            buildMiddlePiece(orangeMaterial, greenMaterial, redMaterial, Math.PI),
        ];

        this.bottomPieces = [
            buildSmallPiece(yellowMaterial, pieceMaterial, orangeMaterial, 6 * smallPieceAngle, -cubeSide / 2),
            buildBigPiece(yellowMaterial, pieceMaterial, blueMaterial, orangeMaterial, 3 * smallPieceAngle, -cubeSide / 2),
            buildSmallPiece(yellowMaterial, pieceMaterial, blueMaterial, 3 * smallPieceAngle, -cubeSide / 2),
            buildBigPiece(yellowMaterial, pieceMaterial, redMaterial, blueMaterial, 0, -cubeSide / 2),
            buildSmallPiece(yellowMaterial, pieceMaterial, redMaterial, 12 * smallPieceAngle, -cubeSide / 2),
            buildBigPiece(yellowMaterial, pieceMaterial, greenMaterial, redMaterial, 9 * smallPieceAngle, -cubeSide / 2),
            buildSmallPiece(yellowMaterial, pieceMaterial, greenMaterial, 9 * smallPieceAngle, -cubeSide / 2),
            buildBigPiece(yellowMaterial, pieceMaterial, orangeMaterial, greenMaterial, 6 * smallPieceAngle, -cubeSide / 2),
        ];

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
    }

    flip() {
        if (this._isMoving) {
            throw Error('Only one movement can be applied at a time');
        }
        this._isMoving = true;

        const [flipTop, stayTop] = this._split(this.topPieces);
        const [flipBottom, stayBottom] = this._split(this.bottomPieces);

        const flipGroup = new THREE.Group();
        this.mesh.add(flipGroup);
        for (const piece of flipTop) {
            flipGroup.attach(piece.mesh);
        }
        flipGroup.attach(this.middlePieces[0].mesh);
        for (const piece of flipBottom) {
            flipGroup.attach(piece.mesh);
        }

        const initialQuaternion = new THREE.Quaternion();
        const angle = Math.PI / 12;
        const axis = new THREE.Vector3(Math.cos(angle), Math.sin(angle), 0);
        const finalQuaternion = initialQuaternion.clone();
        finalQuaternion.setFromAxisAngle(axis, Math.PI);

        const rotationTrack = new THREE.QuaternionKeyframeTrack('.quaternion', [0, 1], [
            initialQuaternion.x,
            initialQuaternion.y,
            initialQuaternion.z,
            initialQuaternion.w,
            finalQuaternion.x,
            finalQuaternion.y,
            finalQuaternion.z,
            finalQuaternion.w,
        ]);

        const animationClip = new THREE.AnimationClip('flip', 1, [rotationTrack]);
        const action = this._mixer.clipAction(animationClip, flipGroup);
        action.setLoop(THREE.LoopOnce, 0);

        const finishMovement = () => {
            this._isMoving = false;
            this._mixer.removeEventListener('finished', finishMovement);

            flipGroup.quaternion.copy(finalQuaternion);
            for (const pieceMesh of flipGroup.children.slice()) {
                this.mesh.attach(pieceMesh);
            }
        };
        this._mixer.addEventListener('finished', finishMovement);

        action.play();
    }

    animate(delta) {
        this._mixer.update(delta);
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

        throw Error('There must be a prefix that adds up to 6');
    }
}

class SmallPiece {
    constructor(halfSide, cubeSide, verticalBevel, horizontalBevel, mainMaterial, bottomFaceMaterial, topFaceMaterial, sideFaceMaterial) {
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

        const piece = new THREE.Mesh(geometry, [bottomFaceMaterial, topFaceMaterial, mainMaterial]);
        piece.translateZ(verticalBevel);
        group.add(piece);

        const sideFaceGeometry = new THREE.PlaneGeometry(2 * halfSide - 2 * horizontalBevel, sideHeight);
        const sideFace = new THREE.Mesh(sideFaceGeometry, sideFaceMaterial);
        sideFace.rotateX(Math.PI / 2);
        sideFace.translateZ(halfCubeSide * 1.001);
        sideFace.translateY(halfCubeSide / 3);
        group.add(sideFace);

        this.mesh = group;

        this.size = 1;
    }
}

class BigPiece {
    constructor(halfSmallPieceSide, cubeSide, verticalBevel, horizontalBevel, mainMaterial, bottomFaceMaterial, topFaceMaterial, sideYFaceMaterial, sideXFaceMaterial) {
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

        const piece = new THREE.Mesh(geometry, [bottomFaceMaterial, topFaceMaterial, mainMaterial]);
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

camera.position.z = 3;

// controls

const controls = new OrbitControls(camera, renderer.domElement);
controls.minDistance = 0.5;
controls.maxDistance = 3;
controls.maxPolarAngle = 3 * Math.PI / 2;

// ambient light

scene.add(new THREE.AmbientLight('white'));

// point light

// const light = new THREE.PointLight( 0xffffff, 1 );
// camera.add( light );

{
    const color = 0xFFFFFF;
    const intensity = 1;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(-1, 2, 6);
    scene.add(light);

    const light2 = new THREE.DirectionalLight(color, intensity);
    light2.position.set(1, -2, 6);
    // scene.add(light2);
}

// helper

cube.mesh.add(new THREE.AxesHelper(20));
scene.add(new THREE.AxesHelper(10));

const clock = new THREE.Clock();

function animate() {
    requestAnimationFrame(animate);
    // cube.mesh.rotation.z -= 0.003;
    // cube.mesh.rotation.x -= 0.001;
    cube.animate(clock.getDelta());
    renderer.render(scene, camera);
}

animate();

document.getElementById('flip').onclick = () => {
    cube.flip();
};