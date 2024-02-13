import sys
import matplotlib.pyplot as plt

def latex_to_image(latex_string, filename):
    """Converts a LaTeX string to an image using Matplotlib with LaTeX rendering.

    Args:
        latex_string (str): The LaTeX string to convert, enclosed in $...$ for math mode.
        filename (str): The name of the output image file.
    """
    
    # Configure Matplotlib to use LaTeX for text rendering
    plt.rc('text', usetex=True)
    plt.rc('text.latex', preamble=r'\usepackage{amsmath,amsfonts,amssymb}')
    plt.rc('font', family='serif')

    # Estimate figure size based on LaTeX string length (basic heuristic)
    fig_width = max(6, len(latex_string) * 0.2)
    fig_height = max(2, len(latex_string) * 0.05)

    fig, ax = plt.subplots(figsize=(fig_width, fig_height))

    # Center the text both horizontally and vertically and render with LaTeX
    ax.text(0.5, 0.5, latex_string, ha="center", va="center", fontsize=20)

    # Remove unnecessary axes
    ax.axis("off")

    # Adjust layout for tight margins
    fig.tight_layout(pad=0.1)

    # Save the image with specified filename
    fig.savefig(filename, dpi=600, bbox_inches='tight')

    plt.close()

if __name__ == "__main__":
    # Check for both arguments
    if len(sys.argv) < 3:
        print("Usage: python script.py <latex_expression> <filename>")
        exit(1)

    # Get latex expression and filename from arguments
    latex_string = sys.argv[1]
    filename = sys.argv[2]

    # Ensure the LaTeX string is enclosed in $...$ for math mode
    if not latex_string.startswith('$'):
        latex_string = f"${latex_string}$"

    # Call the latex_to_image function
    latex_to_image(latex_string, filename)

    print(f"Image generated and saved as: {filename}")
