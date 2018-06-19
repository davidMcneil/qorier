import { SubscriptionMetrics } from "../utils/data_parsers";
import { NotificationType } from "../utils/types";
import { Subscription } from "./subscription";

interface Props {
  subscriptions: SubscriptionMetrics[];
  setNotification: (type: NotificationType, message: string) => void;
  setDeleteConfirmation: (message: string, action: () => void) => void;
}

export function SubscriptionsTable(props: Props) {
  return (
    <table
      class="table table-with-bottom-border is-hoverable is-narrow is-fullwidth"
      $HasKeyedChildren
    >
      {[
        <thead key="header">
          <tr>
            <th />
            <th>Subscription</th>
            <th>Topic</th>
            <th>Pending</th>
            <th>Index</th>
            <th>Processed</th>
            <th>Age</th>
            <th />
          </tr>
        </thead>,
        ...props.subscriptions.map(s => (
          <Subscription
            metrics={s}
            setNotification={props.setNotification}
            setDeleteConfirmation={props.setDeleteConfirmation}
          />
        )),
      ]}
    </table>
  );
}
