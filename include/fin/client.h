#ifndef __FIN_CLIENT_H__
#define __FIN_CLIENT_H__

#include <glib-object.h>

G_BEGIN_DECLS

#define FIN_TYPE_CLIENT            (fin_client_get_type())
#define FIN_CLIENT(obj)            (G_TYPE_CHECK_INSTANCE_CAST((obj),FIN_TYPE_CLIENT,FinClient))
#define FIN_IS_CLIENT(obj)         (G_TYPE_CHECK_INSTANCE_TYPE((obj),FIN_TYPE_CLIENT))
#define FIN_CLIENT_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST((klass) ,FIN_TYPE_CLIENT,FinClientClass))
#define FIN_IS_CLIENT_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE((klass) ,FIN_TYPE_CLIENT))
#define FIN_CLIENT_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj) ,FIN_TYPE_CLIENT,FinClientClass))

typedef struct _FinClient      FinClient;
typedef struct _FinClientClass FinClientClass;

struct _FinClient {
  GObject parent;
};

struct _FinClientClass {
  GObjectClass parent_class;
};

GType fin_client_get_type(void);

FinClient *fin_client_new();

const gchar *fin_client_get_revision(FinClient *client);

G_END_DECLS

#endif /* __FIN_CLIENT_H__ */
