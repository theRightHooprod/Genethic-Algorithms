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
state = {"target_row": 0, "calc_row": 0, "last_dist_row": 0, "generation": 0}

def regenerate_taget_plot():
        ax1.clear()

        df_target = pd.read_csv("target_curve.csv", names=['X', 'Y'])
        ax1.plot(df_target['X'], df_target['Y'], color='red', linestyle=':', alpha=0.7)

def animate(_):
    try:
        df_calc = pd.read_csv("result_curve.csv", names=['rX', 'rY'])
        current_len_calc = len(df_calc)
        
        if current_len_calc > state["calc_row"]:
            regenerate_taget_plot()
            new_data = df_calc.iloc[state["calc_row"]:]
            gen_color = plt.cm.jet((state["generation"] * 15) % 256)
            ax1.plot(new_data['rX'], new_data['rY'], color=gen_color, linestyle='-', alpha=0.7)
            state["calc_row"] = current_len_calc 
            state["generation"] += 1;
    except Exception:
        pass
    try:
        df_dist = pd.read_csv('generation_iteration.csv', names=['iteration', 'fitness'])
        current_dist_len = len(df_dist)
        
        if current_dist_len > state["last_dist_row"]:
            ax2.plot(df_dist['iteration'], df_dist['fitness'], linestyle='-', color="blue", alpha=0.7)
            state["last_dist_row"] = current_dist_len
    except Exception:
        pass

    return []

# Run loop
ani = FuncAnimation(fig, animate, interval=333, cache_frame_data=False)
plt.show()
