/*
1.Load the PNG using image crate.

2.Extract raw pixel data.

3.Split into chunks and compress them in parallel using rayon.

4.Use lz77 crate for actual compression.

5.Save the compressed output to a file. 
 */