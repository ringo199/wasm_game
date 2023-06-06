import init, { World, Direction, GameStatus } from "wasm_game"
import { random } from './utils/random';

init().then((wasm) => {
    const CELL_SIZE = 20;
    const WORLD_WIDTH = 5;
    const snakeIndex = random(WORLD_WIDTH * WORLD_WIDTH)
    const world = World.new(WORLD_WIDTH, snakeIndex);
    const worldWidth = world.width();
    const fps = 6

    const gameStatus = document.getElementById("game-status");
    const gameControlBtn = document.getElementById("game-control-btn");
    const canvas = <HTMLCanvasElement>document.getElementById("snake-world")
    const context = canvas.getContext("2d")

    canvas.width = worldWidth * CELL_SIZE;
    canvas.height = worldWidth * CELL_SIZE;

    gameControlBtn.addEventListener("click", () => {
        const status = world.game_status();
        if (status == undefined) {
            gameControlBtn.textContent = "游戏中...";
            world.start_game();
            run();
        }
        else {
            location.reload();
        }
    })

    document.addEventListener("keydown", e => {
        switch (e.code) {
            case "ArrowUp":
                world.change_snake_direction(Direction.Up);
                break;
            case "ArrowDown":
                world.change_snake_direction(Direction.Down);
                break;
            case "ArrowLeft":
                world.change_snake_direction(Direction.Left);
                break;
            case "ArrowRight":
                world.change_snake_direction(Direction.Right);
                break;
        }
    })

    function drawWorld() {
        context.beginPath()
        for (let x = 0; x < worldWidth + 1; ++x)
        {
            context.moveTo(CELL_SIZE * x, 0)
            context.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth)
        }
        for (let y = 0; y < worldWidth + 1; ++y)
        {
            context.moveTo(0, CELL_SIZE * y)
            context.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y)
        }
        context.stroke()
    }

    function drawSnake() {
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length()
        )
        snakeCells
        .filter((cellIdx, i) => !(i > 0 && cellIdx == snakeCells[0]))
        .forEach((cellIndex, i) => {
            const col = cellIndex % worldWidth;
            const row = Math.floor(cellIndex / worldWidth);

            context.beginPath()
            context.fillStyle = i === 0 ? '#787878' : '#000000'
            context.fillRect(
                col * CELL_SIZE,
                row * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE
            )
        })
        context.stroke()
    }

    function drawReward() {
        const index = world.reward_cell()
        const row = Math.floor(index / worldWidth)
        const col = index % worldWidth

        context.beginPath()
        context.fillStyle = '#ff0000'
        context.fillRect(
            col * CELL_SIZE,
            row * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE
        )
        context.stroke()
    }

    function drawGameStatus() {
        gameStatus.textContent = world.game_status_info();
    }

    function draw()
    {
        drawWorld()
        drawSnake()
        drawReward()
        drawGameStatus()
    }

    function run() {
        const status = world.game_status();
        if (status === GameStatus.Won || status === GameStatus.Lost) {
            gameControlBtn.textContent = "再玩一次？"
            return;
        }
        setTimeout(() => {
            context.clearRect(0, 0, canvas.width, canvas.height)
            world.update()
            draw()
            requestAnimationFrame(run)
        }, 1000 / fps)
    }

    draw()
})