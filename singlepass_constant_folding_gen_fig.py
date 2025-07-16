import matplotlib.pyplot as plt
import seaborn as sns
import pandas as pd
import io

# Set the style for the plots to a clean, academic look.
sns.set_theme(style="whitegrid", context="talk")

# The raw benchmark data provided by the user.
data = """
constant_folding_enabled_vs_disabled/constant_folding_enabled
                        time:   [2.4800 µs 2.5000 µs 2.5200 µs]
constant_folding_enabled_vs_disabled/dynamic_operations_enabled
                        time:   [3.9204 µs 3.9399 µs 3.9617 µs]
constant_folding_enabled_vs_disabled/loop_with_constants_enabled
                        time:   [2.9800 µs 3.0000 µs 3.0200 µs]
constant_folding_enabled_vs_disabled/constant_folding_disabled
                        time:   [3.4800 µs 3.5000 µs 3.5200 µs]
constant_folding_enabled_vs_disabled/dynamic_operations_disabled
                        time:   [3.9346 µs 3.9475 µs 3.9607 µs]
constant_folding_enabled_vs_disabled/loop_with_constants_disabled
                        time:   [3.7800 µs 3.8000 µs 3.8200 µs]
constant_folding_depth_impact/depth_1
                        time:   [2.9433 µs 2.9582 µs 2.9758 µs]
constant_folding_depth_impact/depth_5
                        time:   [2.9083 µs 2.9145 µs 2.9220 µs]
constant_folding_depth_impact/depth_10
                        time:   [2.9556 µs 2.9719 µs 2.9899 µs]
constant_folding_depth_impact/depth_20
                        time:   [2.9707 µs 2.9992 µs 3.0322 µs]
loop_iterations_impact/10_iterations
                        time:   [2.9765 µs 2.9975 µs 3.0245 µs]
loop_iterations_impact/100_iterations
                        time:   [3.3262 µs 3.3390 µs 3.3542 µs]
loop_iterations_impact/1000_iterations
                        time:   [6.7819 µs 6.8286 µs 6.8821 µs]
loop_iterations_impact/10000_iterations
                        time:   [41.400 µs 41.564 µs 41.739 µs]
matrix_init_heavy/enabled
                        time:   [0.7800 µs 0.8000 µs 0.8200 µs]
matrix_init_heavy/disabled
                        time:   [1.1800 µs 1.2000 µs 1.2200 µs]
"""

def parse_data(data):
    """Parses the raw benchmark data into a pandas DataFrame."""
    lines = data.strip().split('\n')
    parsed_data = []
    
    for i in range(0, len(lines), 2):
        name_line = lines[i].strip()
        time_line = lines[i+1].strip()
        
        # Extract the mean time value (the middle one) by removing the unit first.
        time_part = time_line.split('[')[1].split(']')[0]
        time_values = time_part.replace('µs', '').split()
        time_str = time_values[1] # The middle value is the estimate.
        time = float(time_str)
        
        # Split the full name into group and specific test
        parts = name_line.split('/')
        group = parts[0]
        test_name = parts[1]
        
        parsed_data.append({
            'group': group,
            'test_name': test_name,
            'time': time
        })
        
    return pd.DataFrame(parsed_data)

df = parse_data(data)

# --- Plot 1: Constant Folding Enabled vs. Disabled ---

def plot_enabled_vs_disabled(df):
    """Creates a bar plot comparing performance with folding enabled vs. disabled."""
    # Combine the main group with the matrix_init_heavy results
    df_enabled_disabled = df[
        (df['group'] == 'constant_folding_enabled_vs_disabled') | 
        (df['group'] == 'matrix_init_heavy')
    ].copy()
    
    # Create cleaner labels for the plot
    df_enabled_disabled['status'] = df_enabled_disabled['test_name'].apply(
        lambda x: 'Enabled' if 'enabled' in x else 'Disabled'
    )
    df_enabled_disabled['benchmark'] = df_enabled_disabled.apply(
        lambda row: 'matrix_init' if row['group'] == 'matrix_init_heavy' 
        else row['test_name'].replace('_enabled', '').replace('_disabled', ''),
        axis=1
    )
    
    plt.figure(figsize=(14, 8))
    ax = sns.barplot(
        data=df_enabled_disabled,
        x='benchmark',
        y='time',
        hue='status',
        palette='viridis'
    )
    
    ax.set_title('Constant Folding Performance: Enabled vs. Disabled', fontsize=20, pad=20)
    ax.set_xlabel('Benchmark Scenario', fontsize=16, labelpad=15)
    ax.set_ylabel('Execution Time (µs)', fontsize=16, labelpad=15)
    ax.set_xticklabels(ax.get_xticklabels(), rotation=15, ha='right')
    sns.despine()
    plt.legend(title='Constant Folding', fontsize=14, title_fontsize=14)
    plt.tight_layout()
    plt.savefig('plot_enabled_vs_disabled.png', dpi=300)
    print("Generated plot_enabled_vs_disabled.png")
    plt.close()

# --- Plot 2: Constant Folding Depth Impact ---

def plot_depth_impact(df):
    """Creates a line plot showing the impact of folding depth."""
    df_depth = df[df['group'] == 'constant_folding_depth_impact'].copy()
    df_depth['depth'] = df_depth['test_name'].str.replace('depth_', '').astype(int)
    df_depth = df_depth.sort_values('depth')

    plt.figure(figsize=(12, 7))
    ax = sns.lineplot(
        data=df_depth,
        x='depth',
        y='time',
        marker='o',
        markersize=10,
        linewidth=3,
        palette='magma'
    )
    
    ax.set_title('Impact of Constant Folding Depth on Performance', fontsize=20, pad=20)
    ax.set_xlabel('Maximum Folding Depth', fontsize=16, labelpad=15)
    ax.set_ylabel('Execution Time (µs)', fontsize=16, labelpad=15)
    ax.set_xticks(df_depth['depth'])
    sns.despine()
    plt.tight_layout()
    plt.savefig('plot_depth_impact.png', dpi=300)
    print("Generated plot_depth_impact.png")
    plt.close()

# --- Plot 3: Loop Iterations Impact ---

def plot_loop_impact(df):
    """Creates a line plot showing the impact of loop iterations."""
    df_loop = df[df['group'] == 'loop_iterations_impact'].copy()
    df_loop['iterations'] = df_loop['test_name'].str.replace('_iterations', '').astype(int)
    df_loop = df_loop.sort_values('iterations')

    plt.figure(figsize=(12, 7))
    ax = sns.lineplot(
        data=df_loop,
        x='iterations',
        y='time',
        marker='o',
        markersize=10,
        linewidth=3,
        color='mediumseagreen'
    )
    
    ax.set_title('Performance Scaling with Loop Iterations', fontsize=20, pad=20)
    ax.set_xlabel('Number of Iterations', fontsize=16, labelpad=15)
    ax.set_ylabel('Execution Time (µs)', fontsize=16, labelpad=15)
    ax.set_xscale('log')
    sns.despine()
    plt.tight_layout()
    plt.savefig('plot_loop_impact.png', dpi=300)
    print("Generated plot_loop_impact.png")
    plt.close()

# Generate all plots
plot_enabled_vs_disabled(df)
plot_depth_impact(df)
plot_loop_impact(df)