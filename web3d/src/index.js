import * as THREE from 'three';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 5);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

const material1 = new THREE.MeshStandardMaterial({color: 'green'});
const material2 = new THREE.MeshStandardMaterial({color: 'red'});

const cubeSide = 1;

const smallPieceAngle = Math.PI / 6;
const halfSmallPieceSide = cubeSide * Math.tan(smallPieceAngle / 2) / 2;
const halfCubeSide = cubeSide / 2;

function buildSmallPiece(material, topFaceMaterial, sideFaceMaterial) {
    const topShape = new THREE.Shape();
    topShape.moveTo(0, 0);
    topShape.lineTo(-halfSmallPieceSide, -halfCubeSide);
    topShape.lineTo(halfSmallPieceSide, -halfCubeSide);
    topShape.closePath();

    const pieceGroup = new THREE.Group();

    const pieceGeometry = new THREE.ExtrudeGeometry(topShape, {
        depth: cubeSide / 3,
        bevelSize: 0,
        bevelThickness: 0,
    });
    const piece = new THREE.Mesh(pieceGeometry, material);
    pieceGroup.add(piece);

    const topFaceGeometry = new THREE.ShapeGeometry(topShape);
    const topFace = new THREE.Mesh(topFaceGeometry, topFaceMaterial);
    topFace.position.z = cubeSide / 3 * 1.001;
    pieceGroup.add(topFace);

    const sideFaceGeometry = new THREE.PlaneGeometry(2 * halfSmallPieceSide, cubeSide / 3);
    const sideFace = new THREE.Mesh(sideFaceGeometry, sideFaceMaterial);
    sideFace.rotateX(Math.PI / 2);
    sideFace.translateZ(halfCubeSide * 1.001);
    sideFace.translateY(halfCubeSide / 3);
    pieceGroup.add(sideFace);

    return pieceGroup;
}

function buildBigPiece(material, topFaceMaterial, sideXFaceMaterial, sideYFaceMaterial) {
    const topShape = new THREE.Shape();
    topShape.moveTo(0, 0);
    topShape.lineTo(halfSmallPieceSide, -halfCubeSide);
    topShape.lineTo(halfCubeSide, -halfCubeSide);
    topShape.lineTo(halfCubeSide, -halfSmallPieceSide);
    topShape.closePath();

    const pieceGroup = new THREE.Group();

    const pieceGeometry = new THREE.ExtrudeGeometry(topShape, {
        depth: cubeSide / 3,
        bevelSize: 0,
        bevelThickness: 0,
    });
    const piece = new THREE.Mesh(pieceGeometry, material);
    pieceGroup.add(piece);

    const topFaceGeometry = new THREE.ShapeGeometry(topShape);
    const topFace = new THREE.Mesh(topFaceGeometry, topFaceMaterial);
    topFace.position.z = cubeSide / 3 * 1.001;
    pieceGroup.add(topFace);

    return pieceGroup;
}

const cube = new THREE.Group();

const pieceMaterial = new THREE.MeshStandardMaterial({color: 'blue'});
const whiteMaterial = new THREE.MeshStandardMaterial({color: 'white'});
const redMaterial = new THREE.MeshStandardMaterial({color: 'red'});
const greenMaterial = new THREE.MeshStandardMaterial({color: 'green'});

cube.add(buildSmallPiece(pieceMaterial, whiteMaterial, redMaterial));
cube.add(buildBigPiece(pieceMaterial, whiteMaterial, redMaterial, greenMaterial));

scene.add(cube);

camera.position.z = 4;

// controls

// const controls = new OrbitControls( camera, renderer.domElement );
// controls.minDistance = 10;
// controls.maxDistance = 20;
// controls.maxPolarAngle = Math.PI / 2;

// ambient light

// scene.add( new THREE.AmbientLight( 0x222222 ) );

// point light

// const light = new THREE.PointLight( 0xffffff, 1 );
// camera.add( light );

{
    const color = 0xFFFFFF;
    const intensity = 1;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(-1, 2, 4);
    scene.add(light);
}

// helper

scene.add(new THREE.AxesHelper(20));

function animate() {
    requestAnimationFrame(animate);
    cube.rotation.x -= 0.01;
    cube.rotation.y -= 0.01;
    renderer.render(scene, camera);
}

animate();