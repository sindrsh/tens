import inh;

size(100*to_pixel*cm);

real h = sqrt(3);
pair A = (0,0);
pair B = (2,0);
pair C = (1,h);

filldraw(A--B--C--cycle,blue);
