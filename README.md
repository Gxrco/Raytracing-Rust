
# Raytracer Scene: Snowy Night with Igloo

## Overview

This project is a raytracer implementation showcasing a visually compelling snowy night scene. It features an igloo constructed with intercalated ice and packed ice blocks with platform randomly generated to have "different" locations anytime, a glowing environment with light-emitting materials, and a skybox that sets the tone for the snowy ambiance.

## Features

### Scene Complexity and Visual Appeal
- The scene is composed of multiple layers of blocks, including intercalated ice and packed ice materials, providing depth and realism to the igloo construction.
- Glowstone blocks, emitting warm light, enhance the visual contrast against the cold snow and ice textures.
- A realistic skybox with a snowy landscape immerses the viewer in the environment.
- Performance has been improved to show better results.

### Material Variety
- Multiple materials with unique textures and properties:
  - **Snow**: Highly diffuse, cold appearance.
  - **Stone**: Subtle reflective properties for added realism.
  - **Ice**: Transparent with refractive index adjustments for realism.
  - **Packed Ice**: Solid surface and adjusted transparency for differentiation.
  - **Glowstone**: Emissive material with a warm glow, acting as a light source.
- Each material has distinct parameters for albedo, specularity, transparency, and reflectivity.

### Advanced Lighting
- Support for multiple light sources of varying colors and intensities:
  - A natural-looking light system includes soft, cool, and warm tones for contrast.
  - Glowstone blocks contribute as emissive light sources, dynamically affecting shadows and lighting.

### Camera Interactions
- A modified camera system allows zooming in and out while maintaining focus on the igloo, enabling detailed exploration of the scene.

### Realism Enhancements
- **Fresnel Effect**: Enhances reflection and refraction calculations for transparent materials, such as ice and packed ice.
- **Normal Mapping**: Adds apparent detail to surfaces without increasing geometric complexity, making textures more vivid and realistic.

### Additional Features
- **Skybox Integration**: A custom snowy skybox provides a realistic backdrop.
- **Dynamic Scene Elements**:
  - Materials like glowstone dynamically influence the lighting and shadows.
  - Intercalation logic for block placement creates a visually varied structure.

## Conclusion

This raytracer demonstrates a robust implementation of advanced rendering techniques, realistic material properties, and dynamic scene interactions. The interplay of light, texture, and geometry results in a visually engaging environment that captures the serenity and depth of a snowy night.


## Demo

https://github.com/user-attachments/assets/b9a22555-ae5d-4369-a481-ade6a76d9e94

