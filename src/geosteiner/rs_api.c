#include "geosteiner.h"
#include "stdlib.h"

struct C_ESMT {
double length;
int nsps;
double * sps;
};

struct C_ESMT rs_compute_esmt (int nterms, double * terms)
    {
    int i, nsps;
    double length, sps [4];

    /* Open GeoSteiner environment */
    if (gst_open_geosteiner () != 0) {
        printf ("Could not open GeoSteiner.\n");
        exit (1);
    }

    /* Compute Euclidean Steiner tree */
    gst_esmt (nterms, terms, &length, &nsps, sps, NULL, NULL, NULL, NULL);

    for (i = 0; i < nsps; i++) {
        printf ("Steiner point: (%f, %f)\n", sps[2*i], sps[2*i+1]);
    }

    /* Close GeoSteiner environment */
    gst_close_geosteiner ();

    struct C_ESMT c_esmt;
    c_esmt.length = length;
    c_esmt.nsps = nsps;
    c_esmt.sps = sps;

	return c_esmt;
}
