#ifndef GEOSTEINER_H
#define GEOSTEINER_H

#include <stdlib.h>
#include <stdio.h>


#endif

typedef struct gst_param * gst_param_ptr;

int gst_open_geosteiner (void);

int gst_close_geosteiner (void);

int gst_esmt (int            nterms, 
              double*        terms, 
              double*        length,
              int*           nsps,  
              double*        sps, 
              int*           nedges,
              int*           edges, 
              int*           status,
              gst_param_ptr  param);
