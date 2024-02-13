import sys
import matplotlib.pyplot as plt

def latex_to_image(latex_string, filename):
    """Converts a LaTeX string to an image using Matplotlib.

    Args:
        latex_string (str): The LaTeX string to convert.
        filename (str): The name of the output image file.
    """

    fig, ax = plt.subplots()

    # Center the text both horizontally and vertically
    ax.text(0.5, 0.5, latex_string, ha="center", va="center", fontsize=20)

    # Remove unnecessary axes
    ax.axis("off")

    # Adjust layout for tight margins
    fig.tight_layout()

    # Save the image with specified filename
    fig.savefig(filename, dpi=600)

    plt.close()

if __name__ == "__main__":
    # Check for both arguments
    if len(sys.argv) < 3:
        print("Usage: python script.py <latex_expression> <filename>")
        exit(1)

    # Get latex expression and filename from arguments
    latex_string = sys.argv[1]
    filename = sys.argv[2]

    # Call the latex_to_image function
    latex_to_image(latex_string, filename)

    print(f"Image generated and saved as: {filename}")