import matplotlib.pyplot as plt
import pandas as pd
from matplotlib.animation import FuncAnimation

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

# Setup static plot traits once outside loop
ax1.set_title('Best individual value')
ax1.grid(True, alpha=0.3)
ax2.set_title('Best individual score across generations')
ax2.grid(True, alpha=0.3)

# Track how many points processed
state = {"last_points_row": 0, "last_dist_row": 0, "generation": 0}

def animate(_):
    try:
        df_points = pd.read_csv('target_curve.csv', names=['X', 'Y'])
        current_len = len(df_points)
        
        # Only plot if new data arrived
        if current_len > state["last_points_row"]:
            # Slice new chunk
            new_data = df_points.iloc[state["last_points_row"]:]
            
            # Unique color for this generation batch
            gen_color = plt.cm.jet((state["generation"] * 15) % 256)
            
            # Plot new segment without clearing old ones
            ax1.plot(new_data['X'], new_data['Y'], color='red', linestyle=':', alpha=0.7)
            
            state["last_points_row"] = current_len
            state["generation"] += 1
    except Exception:
        pass

    try:
        df_dist = pd.read_csv('generation_iteration.csv', names=['iteration', 'fitness'])
        current_dist_len = len(df_dist)
        
        if current_dist_len > state["last_dist_row"]:
            # Re-draw full distance metric line sequentially
            ax2.clear()
            ax2.grid(True, alpha=0.3)
            ax2.set_title('Distance before vs after reproduction')
            
            # Use colormap to color time steps
            ax2.scatter(df_dist['iteration'], df_dist['fitness'], color = "blue", cmap='jet', s=10)
            
            state["last_dist_row"] = current_dist_len
    except Exception:
        pass

    return []

# Run loop
ani = FuncAnimation(fig, animate, interval=333, cache_frame_data=False)
plt.show()
