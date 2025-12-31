import pygame
import random
import sys

# 初期化
pygame.init()

# 色の定義
BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
CYAN = (0, 255, 255)
BLUE = (0, 0, 255)
ORANGE = (255, 165, 0)
YELLOW = (255, 255, 0)
GREEN = (0, 255, 0)
PURPLE = (128, 0, 128)
RED = (255, 0, 0)
GRAY = (128, 128, 128)

# ゲーム設定
GRID_WIDTH = 10
GRID_HEIGHT = 20
CELL_SIZE = 30
WINDOW_WIDTH = GRID_WIDTH * CELL_SIZE + 200
WINDOW_HEIGHT = GRID_HEIGHT * CELL_SIZE + 100

# テトリミノの形状定義
TETRIMINOS = {
    'I': [
        ['.....',
         '..#..',
         '..#..',
         '..#..',
         '..#..'],
        ['.....',
         '.....',
         '####.',
         '.....',
         '.....']
    ],
    'O': [
        ['.....',
         '.....',
         '.##..',
         '.##..',
         '.....']
    ],
    'T': [
        ['.....',
         '.....',
         '.#...',
         '###..',
         '.....'],
        ['.....',
         '.....',
         '.#...',
         '.##..',
         '.#...'],
        ['.....',
         '.....',
         '.....',
         '###..',
         '.#...'],
        ['.....',
         '.....',
         '.#...',
         '##...',
         '.#...']
    ],
    'S': [
        ['.....',
         '.....',
         '.##..',
         '##...',
         '.....'],
        ['.....',
         '.#...',
         '.##..',
         '..#..',
         '.....']
    ],
    'Z': [
        ['.....',
         '.....',
         '##...',
         '.##..',
         '.....'],
        ['.....',
         '..#..',
         '.##..',
         '.#...',
         '.....']
    ],
    'J': [
        ['.....',
         '.#...',
         '.#...',
         '##...',
         '.....'],
        ['.....',
         '.....',
         '#....',
         '###..',
         '.....'],
        ['.....',
         '.##..',
         '.#...',
         '.#...',
         '.....'],
        ['.....',
         '.....',
         '###..',
         '..#..',
         '.....']
    ],
    'L': [
        ['.....',
         '..#..',
         '..#..',
         '.##..',
         '.....'],
        ['.....',
         '.....',
         '###..',
         '#....',
         '.....'],
        ['.....',
         '##...',
         '.#...',
         '.#...',
         '.....'],
        ['.....',
         '.....',
         '..#..',
         '###..',
         '.....']
    ]
}

TETRIMINO_COLORS = {
    'I': CYAN,
    'O': YELLOW,
    'T': PURPLE,
    'S': GREEN,
    'Z': RED,
    'J': BLUE,
    'L': ORANGE
}

class Tetrimino:
    def __init__(self, shape):
        self.shape = shape
        self.color = TETRIMINO_COLORS[shape]
        self.x = GRID_WIDTH // 2 - 2
        self.y = 0
        self.rotation = 0
        self.grid = TETRIMINOS[shape][self.rotation]
    
    def get_rotated_grid(self, rotation=None):
        if rotation is None:
            rotation = self.rotation
        return TETRIMINOS[self.shape][rotation % len(TETRIMINOS[self.shape])]
    
    def rotate(self):
        self.rotation = (self.rotation + 1) % len(TETRIMINOS[self.shape])
        self.grid = self.get_rotated_grid()
    
    def get_cells(self):
        cells = []
        for y, row in enumerate(self.grid):
            for x, cell in enumerate(row):
                if cell == '#':
                    cells.append((self.x + x, self.y + y))
        return cells

class TetrisGame:
    def __init__(self):
        self.grid = [[BLACK for _ in range(GRID_WIDTH)] for _ in range(GRID_HEIGHT)]
        self.current_piece = self.get_new_piece()
        self.next_piece = self.get_new_piece()
        self.score = 0
        self.lines_cleared = 0
        self.level = 1
        self.fall_time = 0
        self.fall_speed = 500  # ミリ秒
        self.game_over = False
        
        self.screen = pygame.display.set_mode((WINDOW_WIDTH, WINDOW_HEIGHT))
        pygame.display.set_caption("テトリス")
        self.clock = pygame.time.Clock()
        self.font = pygame.font.Font(None, 36)
    
    def get_new_piece(self):
        shape = random.choice(list(TETRIMINOS.keys()))
        return Tetrimino(shape)
    
    def is_valid_position(self, piece, dx=0, dy=0, rotation=None):
        if rotation is not None:
            grid = piece.get_rotated_grid(rotation)
        else:
            grid = piece.grid
        
        for y, row in enumerate(grid):
            for x, cell in enumerate(row):
                if cell == '#':
                    new_x = piece.x + x + dx
                    new_y = piece.y + y + dy
                    
                    if (new_x < 0 or new_x >= GRID_WIDTH or 
                        new_y >= GRID_HEIGHT or
                        (new_y >= 0 and self.grid[new_y][new_x] != BLACK)):
                        return False
        return True
    
    def place_piece(self, piece):
        for x, y in piece.get_cells():
            if y >= 0:
                self.grid[y][x] = piece.color
        
        lines_to_clear = []
        for y in range(GRID_HEIGHT):
            if all(cell != BLACK for cell in self.grid[y]):
                lines_to_clear.append(y)
        
        for y in lines_to_clear:
            del self.grid[y]
            self.grid.insert(0, [BLACK for _ in range(GRID_WIDTH)])
        
        lines_cleared = len(lines_to_clear)
        self.lines_cleared += lines_cleared
        self.score += lines_cleared * 100 * self.level
        self.level = self.lines_cleared // 10 + 1
        self.fall_speed = max(50, 500 - (self.level - 1) * 50)
    
    def move_piece(self, dx, dy):
        if self.is_valid_position(self.current_piece, dx, dy):
            self.current_piece.x += dx
            self.current_piece.y += dy
            return True
        return False
    
    def rotate_piece(self):
        new_rotation = (self.current_piece.rotation + 1) % len(TETRIMINOS[self.current_piece.shape])
        if self.is_valid_position(self.current_piece, rotation=new_rotation):
            self.current_piece.rotate()
            return True
        return False
    
    def update(self, dt):
        if self.game_over:
            return
        
        self.fall_time += dt
        if self.fall_time >= self.fall_speed:
            if not self.move_piece(0, 1):
                self.place_piece(self.current_piece)
                self.current_piece = self.next_piece
                self.next_piece = self.get_new_piece()
                
                if not self.is_valid_position(self.current_piece):
                    self.game_over = True
            
            self.fall_time = 0
    
    def draw_grid(self):
        for y in range(GRID_HEIGHT):
            for x in range(GRID_WIDTH):
                rect = pygame.Rect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
                pygame.draw.rect(self.screen, self.grid[y][x], rect)
                pygame.draw.rect(self.screen, GRAY, rect, 1)
    
    def draw_piece(self, piece):
        for x, y in piece.get_cells():
            if y >= 0:
                rect = pygame.Rect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
                pygame.draw.rect(self.screen, piece.color, rect)
                pygame.draw.rect(self.screen, WHITE, rect, 1)
    
    def draw_next_piece(self):
        start_x = GRID_WIDTH * CELL_SIZE + 20
        start_y = 50
        
        text = self.font.render("Next:", True, WHITE)
        self.screen.blit(text, (start_x, start_y - 30))
        
        for y, row in enumerate(self.next_piece.grid):
            for x, cell in enumerate(row):
                if cell == '#':
                    rect = pygame.Rect(start_x + x * 20, start_y + y * 20, 20, 20)
                    pygame.draw.rect(self.screen, self.next_piece.color, rect)
                    pygame.draw.rect(self.screen, WHITE, rect, 1)
    
    def draw_info(self):
        start_x = GRID_WIDTH * CELL_SIZE + 20
        start_y = 200
        
        score_text = self.font.render(f"Score: {self.score}", True, WHITE)
        level_text = self.font.render(f"Level: {self.level}", True, WHITE)
        lines_text = self.font.render(f"Lines: {self.lines_cleared}", True, WHITE)
        
        self.screen.blit(score_text, (start_x, start_y))
        self.screen.blit(level_text, (start_x, start_y + 40))
        self.screen.blit(lines_text, (start_x, start_y + 80))
        
        if self.game_over:
            game_over_text = self.font.render("GAME OVER", True, RED)
            restart_text = self.font.render("Press R to restart", True, WHITE)
            self.screen.blit(game_over_text, (start_x, start_y + 140))
            self.screen.blit(restart_text, (start_x, start_y + 180))
    
    def draw(self):
        self.screen.fill(BLACK)
        self.draw_grid()
        if not self.game_over:
            self.draw_piece(self.current_piece)
        self.draw_next_piece()
        self.draw_info()
        pygame.display.flip()
    
    def restart(self):
        self.grid = [[BLACK for _ in range(GRID_WIDTH)] for _ in range(GRID_HEIGHT)]
        self.current_piece = self.get_new_piece()
        self.next_piece = self.get_new_piece()
        self.score = 0
        self.lines_cleared = 0
        self.level = 1
        self.fall_time = 0
        self.fall_speed = 500
        self.game_over = False
    
    def handle_input(self, event):
        if event.type == pygame.KEYDOWN:
            if self.game_over:
                if event.key == pygame.K_r:
                    self.restart()
            else:
                if event.key == pygame.K_LEFT:
                    self.move_piece(-1, 0)
                elif event.key == pygame.K_RIGHT:
                    self.move_piece(1, 0)
                elif event.key == pygame.K_DOWN:
                    self.move_piece(0, 1)
                elif event.key == pygame.K_UP:
                    self.rotate_piece()
                elif event.key == pygame.K_SPACE:
                    while self.move_piece(0, 1):
                        pass
    
    def run(self):
        running = True
        while running:
            dt = self.clock.tick(60)
            
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                else:
                    self.handle_input(event)
            
            self.update(dt)
            self.draw()
        
        pygame.quit()
        sys.exit()

if __name__ == "__main__":
    game = TetrisGame()
    game.run()
